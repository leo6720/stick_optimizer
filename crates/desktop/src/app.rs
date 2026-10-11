use eframe::egui::{self, Color32, Id, RichText, TextEdit};
use egui_extras::{Column, TableBuilder};
use std::collections::HashSet;
use std::sync::{Arc, atomic::Ordering};
use std::thread::JoinHandle;
use stick_optimizer_core::{
    SearchControl, SearchError, SearchLimits, SearchReport, Solution, optimize,
};

use crate::draft::{StudyDraft, parse_decimal};

const ACCENT: Color32 = Color32::from_rgb(181, 31, 32);
const MUTED: Color32 = Color32::from_rgb(80, 85, 85);

#[derive(Clone, Copy, PartialEq, Eq)]
enum DetailView {
    Formats,
    Pockets,
    Heads,
}

#[derive(Clone, Copy)]
enum InputRow {
    Stick(usize),
    Format(usize),
}

#[derive(Clone, Copy)]
enum PendingAction {
    EmptyStudy,
    ExampleStudy,
    RemoveStick(u64),
}

#[derive(Clone, Debug)]
struct ResultRow {
    id: usize,
    pitch: f64,
    pockets: usize,
    heads: usize,
    layers: u32,
    score: f64,
    solution: Arc<Solution>,
}

struct SearchJob {
    revision: u64,
    control: Arc<SearchControl>,
    handle: Option<JoinHandle<Result<SearchReport, SearchError>>>,
}

impl Drop for SearchJob {
    fn drop(&mut self) {
        self.control.cancelled.store(true, Ordering::Relaxed);
    }
}

pub struct PrototypeApp {
    study: StudyDraft,
    parameters_open: bool,
    detail_view: DetailView,
    collapsed: HashSet<u64>,
    selected_stick: Option<u64>,
    mt_image: Option<egui::TextureHandle>,
    stick_image: Option<egui::TextureHandle>,
    results: Vec<ResultRow>,
    selected: Option<usize>,
    one_head_only: bool,
    order_by_pitch: bool,
    pending: Option<PendingAction>,
    status: String,
    revision: u64,
    job: Option<SearchJob>,
    diagnostic: Option<String>,
}

impl Default for PrototypeApp {
    fn default() -> Self {
        Self {
            study: StudyDraft::example(),
            parameters_open: false,
            detail_view: DetailView::Formats,
            collapsed: HashSet::new(),
            selected_stick: None,
            mt_image: None,
            stick_image: None,
            results: Vec::new(),
            selected: None,
            one_head_only: false,
            order_by_pitch: false,
            pending: None,
            status: "Dati di esempio pronti. Premi Calcola per cercare soluzioni reali.".into(),
            revision: 0,
            job: None,
            diagnostic: None,
        }
    }
}

impl PrototypeApp {
    pub fn new(ctx: &egui::Context) -> Self {
        ctx.set_visuals(egui::Visuals::light());
        ctx.global_style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(6.0, 6.0);
            style.spacing.button_padding = egui::vec2(10.0, 5.0);
            style.visuals.selection.bg_fill = ACCENT;
            style.visuals.selection.stroke.color = Color32::WHITE;
            style.visuals.widgets.active.bg_fill = ACCENT;
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(248, 233, 233);
            style.visuals.widgets.noninteractive.fg_stroke.color = MUTED;
            style.visuals.widgets.inactive.bg_fill = Color32::WHITE;
            style.visuals.widgets.inactive.bg_stroke.color = Color32::from_rgb(210, 212, 212);
            style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(2);
            style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(2);
            style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(2);
            style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(2);
            style
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(14.0));
            style
                .text_styles
                .insert(egui::TextStyle::Button, egui::FontId::proportional(14.0));
            style
                .text_styles
                .insert(egui::TextStyle::Heading, egui::FontId::proportional(21.0));
        });
        #[allow(unused_mut)] // Preview modes mutate the initial state only in QA builds.
        let mut app = Self {
            mt_image: load_diagram(
                ctx,
                "mt_diagram",
                include_bytes!("../../../img/dati_mt.jpg"),
            ),
            stick_image: load_diagram(
                ctx,
                "stick_diagram",
                include_bytes!("../../../img/stick_dim.jpg"),
            ),
            ..Self::default()
        };
        #[cfg(feature = "ui-preview")]
        match std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref() {
            Ok("results" | "compact") => {
                app.start_search();
            }
            Ok("pockets") => {
                app.detail_view = DetailView::Pockets;
                app.start_search();
            }
            Ok("heads") => {
                app.detail_view = DetailView::Heads;
                app.start_search();
            }
            Ok("no-candidates") => {
                app.study.formats[2].quantity = "50".into();
                app.start_search();
            }
            Ok("no-common") => {
                app.study.parameters.offset = "0".into();
                app.study.sticks[1].width = "52,5".into();
                app.start_search();
            }
            Ok("parameters") => {
                app.parameters_open = true;
            }
            Ok("scaled") => {
                ctx.set_zoom_factor(1.5);
                app.start_search();
            }
            Ok("many") => {
                for _ in 0..118 {
                    app.study.add_stick();
                    app.study.add_format();
                }
            }
            _ => {}
        }
        app
    }

    fn invalidate_results(&mut self) {
        self.results.clear();
        self.selected = None;
        self.diagnostic = None;
        self.revision += 1;
        if let Some(job) = &self.job {
            job.control.cancelled.store(true, Ordering::Relaxed);
        }
        self.status = "Dati modificati. Premi Calcola per aggiornare i risultati.".into();
    }

    fn start_search(&mut self) -> bool {
        if self.job.is_some() {
            return false;
        }
        let study = match self.study.to_study() {
            Ok(study) => study,
            Err(errors) => {
                self.diagnostic = Some(errors.join("; "));
                return false;
            }
        };
        self.invalidate_results();
        let control = Arc::new(SearchControl::default());
        let worker_control = Arc::clone(&control);
        match std::thread::Builder::new()
            .name("stick-search".into())
            .spawn(move || optimize(&study, &worker_control, SearchLimits::default()))
        {
            Ok(handle) => {
                self.job = Some(SearchJob {
                    revision: self.revision,
                    control,
                    handle: Some(handle),
                });
                self.status = "Generazione candidati…".into();
                true
            }
            Err(error) => {
                self.diagnostic = Some(format!("Impossibile avviare il calcolo: {error}"));
                false
            }
        }
    }

    fn poll_search(&mut self) {
        let Some(job) = &self.job else {
            return;
        };
        if !job.handle.as_ref().expect("active handle").is_finished() {
            self.status = if job.control.cancelled.load(Ordering::Relaxed) {
                "Annullamento del calcolo…".into()
            } else {
                let estimated = job.control.estimated.load(Ordering::Relaxed);
                if estimated == 0 {
                    "Generazione candidati…".into()
                } else {
                    format!(
                        "Calcolo: {} / {estimated} combinazioni",
                        job.control.examined.load(Ordering::Relaxed)
                    )
                }
            };
            return;
        }
        let mut job = self.job.take().expect("job exists");
        let outcome = job.handle.take().expect("active handle").join();
        if job.revision != self.revision {
            self.status = "Input modificati: il calcolo precedente è stato scartato.".into();
            return;
        }
        if job.control.cancelled.load(Ordering::Relaxed) {
            self.status = SearchError::Cancelled.to_string();
            return;
        }
        match outcome {
            Ok(Ok(report)) => self.install_report(report),
            Ok(Err(error)) => {
                self.status = error.to_string();
                self.diagnostic = Some(error.to_string());
            }
            Err(_) => {
                self.status = "Il calcolo è terminato con un errore interno.".into();
                self.diagnostic = Some(self.status.clone());
            }
        }
    }

    fn install_report(&mut self, report: SearchReport) {
        self.status = format!(
            "Ricerca completata: {} combinazioni, {} soluzioni Pareto, {} mostrate.",
            report.examined,
            report.pareto_count,
            report.solutions.len()
        );
        self.results = report
            .solutions
            .into_iter()
            .enumerate()
            .map(|(i, solution)| ResultRow {
                id: i + 1,
                pitch: solution.pitch.mm(),
                pockets: solution.pocket_types.len(),
                heads: solution.head_types.len(),
                layers: solution.max_layers,
                score: solution.score,
                solution: Arc::new(solution),
            })
            .collect();
        self.selected = self.results.first().map(|r| r.id);
        self.one_head_only = false;
        self.order_by_pitch = false;
    }

    fn apply_pending(&mut self, action: PendingAction) {
        match action {
            PendingAction::EmptyStudy => {
                self.study = StudyDraft::default();
                self.collapsed.clear();
                self.selected_stick = None;
            }
            PendingAction::ExampleStudy => {
                self.study = StudyDraft::example();
                self.collapsed.clear();
                self.selected_stick = None;
            }
            PendingAction::RemoveStick(id) => {
                self.study.remove_stick(id);
            }
        }
        self.pending = None;
        self.invalidate_results();
    }

    fn visible_results(&self) -> Vec<ResultRow> {
        let mut rows: Vec<_> = self
            .results
            .iter()
            .filter(|row| !self.one_head_only || row.heads == 1)
            .cloned()
            .collect();
        rows.sort_by(|a, b| {
            let (left, right) = if self.order_by_pitch {
                (a.pitch, b.pitch)
            } else {
                (a.score, b.score)
            };
            left.total_cmp(&right).then(a.id.cmp(&b.id))
        });
        rows
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        self.poll_search();
        if self.job.is_some() {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(50));
        }
        egui::Panel::top("menu_bar")
            .frame(egui::Frame::new().fill(Color32::WHITE).inner_margin(10))
            .show(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Nuovo studio").clicked() {
                            self.pending = Some(PendingAction::EmptyStudy);
                            ui.close();
                        }
                        if ui.button("Carica esempio").clicked() {
                            self.pending = Some(PendingAction::ExampleStudy);
                            ui.close();
                        }
                    });
                    ui.menu_button("Modifica", |ui| {
                        if ui.button("Parametri astucciatrice e ricerca…").clicked() {
                            self.parameters_open = true;
                            ui.close();
                        }
                    });
                    ui.separator();
                    ui.label(RichText::new("Stick Optimizer").strong().color(ACCENT));
                    if ui
                        .add(
                            TextEdit::singleline(&mut self.study.name)
                                .id(Id::new("study_name"))
                                .desired_width(240.0),
                        )
                        .changed()
                    {
                        self.invalidate_results();
                    }
                    ui.label(RichText::new("Beta Rust").small().color(MUTED));
                });
            });
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(242, 242, 242))
                    .inner_margin(8),
            )
            .show(ui, |ui| {
                ui.label(&self.status);
            });

        let maximum_sidebar = (ui.available_width() * 0.5).max(300.0);
        egui::Panel::left("input_sidebar")
            .default_size(430.0)
            .size_range(300.0..=maximum_sidebar)
            .resizable(true)
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(244, 244, 244))
                    .inner_margin(14),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("sidebar_scroll")
                    .show(ui, |ui| {
                        ui.heading(RichText::new("Dati MT").color(ACCENT));
                        if let Some(image) = &self.mt_image {
                            ui.add(
                                egui::Image::new(image).fit_to_exact_size(egui::vec2(210.0, 136.6)),
                            );
                        }
                        ui.horizontal(|ui| {
                            let label = ui.label("Np · Canali MT");
                            if ui
                                .add(
                                    TextEdit::singleline(&mut self.study.parameters.channels)
                                        .id(Id::new(("param", "Canali MT")))
                                        .desired_width(70.0),
                                )
                                .labelled_by(label.id)
                                .changed()
                            {
                                self.invalidate_results();
                            }
                        });
                        ui.separator();
                        ui.heading(RichText::new("Formati").color(ACCENT));
                        ui.horizontal(|ui| {
                            if let Some(image) = &self.stick_image {
                                ui.add(
                                    egui::Image::new(image)
                                        .fit_to_exact_size(egui::vec2(92.0, 106.0)),
                                );
                            }
                            ui.label(
                                RichText::new(
                                    "Hs · Lunghezza stick
As · Larghezza stick
Ss · Spessore stick
Bs · Lunghezza pinna
Dimensioni in mm",
                                )
                                .small()
                                .color(MUTED),
                            );
                        });
                        if self.inputs(ui) {
                            self.invalidate_results();
                        }
                        ui.add_space(8.0);
                        let errors = self.study.input_errors();
                        if !errors.is_empty() {
                            ui.label(RichText::new("Dati da correggere").strong().color(ACCENT));
                            for error in errors.iter().take(5) {
                                ui.label(error);
                            }
                            if errors.len() > 5 {
                                ui.label(format!(
                                    "Altri {} campi da correggere.",
                                    errors.len() - 5
                                ));
                            }
                        }
                        ui.label(
                            RichText::new("Modifiche conservate solo nella sessione.")
                                .small()
                                .color(MUTED),
                        );
                    });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::WHITE).inner_margin(18))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add_enabled(
                                self.job.is_none() && self.study.input_errors().is_empty(),
                                egui::Button::new(RichText::new("Calcola").color(Color32::WHITE))
                                    .fill(ACCENT),
                            )
                            .clicked()
                            && self.job.is_none()
                        {
                            self.start_search();
                        }
                        if let Some(job) = &self.job {
                            if ui.button("Annulla calcolo").clicked() {
                                job.control.cancelled.store(true, Ordering::Relaxed);
                            }
                            ui.spinner();
                        }
                    });
                });
                ui.label(
                    RichText::new("Ricerca geometrica statica · quote in millimetri")
                        .small()
                        .color(MUTED),
                );
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_salt("output_scroll")
                    .show(ui, |ui| {
                        self.results_ui(ui);
                    });
            });
        self.parameters_window(ui.ctx());
        self.confirm_action(ui.ctx());
    }

    fn input_rows(&self) -> Vec<InputRow> {
        let mut children = std::collections::HashMap::<u64, Vec<usize>>::new();
        for (index, format) in self.study.formats.iter().enumerate() {
            if let Some(parent) = format.stick_id {
                children.entry(parent).or_default().push(index);
            }
        }
        let mut rows = Vec::new();
        for (index, stick) in self.study.sticks.iter().enumerate() {
            rows.push(InputRow::Stick(index));
            if !self.collapsed.contains(&stick.id) {
                for &index in children.get(&stick.id).into_iter().flatten() {
                    rows.push(InputRow::Format(index));
                }
            }
        }
        rows
    }

    fn inputs(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        let mut remove_format = None;
        let rows = self.input_rows();
        let width = ui.available_width().max(380.0);
        egui::ScrollArea::horizontal()
            .id_salt("input_horizontal")
            .show(ui, |ui| {
                ui.set_width(width);
                TableBuilder::new(ui)
                    .id_salt("hierarchical_inputs")
                    .striped(true)
                    .column(Column::remainder().at_least(130.0).clip(true))
                    .column(Column::exact(40.0))
                    .columns(Column::exact(43.0), 4)
                    .max_scroll_height(240.0)
                    .sense(egui::Sense::CLICK)
                    .header(26.0, |mut header| {
                        for label in ["Stick / Formato", "Qtà", "Hs", "As", "Ss", "Bs"] {
                            header.col(|ui| {
                                ui.strong(label);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(30.0, rows.len(), |mut row| match rows[row.index()] {
                            InputRow::Stick(index) => {
                                let stick = &mut self.study.sticks[index];
                                let id = stick.id;
                                row.set_selected(self.selected_stick == Some(id));
                                row.col(|ui| {
                                    ui.horizontal(|ui| {
                                        let closed = self.collapsed.contains(&id);
                                        if ui
                                            .add_sized(
                                                [18.0, 22.0],
                                                egui::Button::new(if closed { "+" } else { "−" }),
                                            )
                                            .on_hover_text("Espandi o comprimi i formati")
                                            .clicked()
                                        {
                                            if closed {
                                                self.collapsed.remove(&id);
                                            } else {
                                                self.collapsed.insert(id);
                                            }
                                        }
                                        changed |= cell(ui, ("stick_name", id), &mut stick.name);
                                        if ui.memory(|memory| memory.focused())
                                            == Some(Id::new(("stick_name", id)))
                                        {
                                            self.selected_stick = Some(id);
                                        }
                                    });
                                });
                                row.col(|ui| {
                                    ui.label("—");
                                });
                                for (key, value) in [
                                    ("length", &mut stick.length),
                                    ("width", &mut stick.width),
                                    ("thickness", &mut stick.thickness),
                                    ("fin", &mut stick.fin),
                                ] {
                                    row.col(|ui| {
                                        changed |= cell(ui, (key, id), value);
                                        if ui.memory(|memory| memory.focused())
                                            == Some(Id::new((key, id)))
                                        {
                                            self.selected_stick = Some(id);
                                        }
                                    });
                                }
                                let response = row.response();
                                if response.clicked() {
                                    self.selected_stick = Some(id);
                                }
                                response.context_menu(|ui| {
                                    if ui.button("Rimuovi stick e formati associati").clicked() {
                                        self.pending = Some(PendingAction::RemoveStick(id));
                                        ui.close();
                                    }
                                });
                            }
                            InputRow::Format(index) => {
                                let format = &mut self.study.formats[index];
                                let id = format.id;
                                let parent = format.stick_id;
                                row.col(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.add_space(24.0);
                                        changed |= cell(ui, ("format_name", id), &mut format.name);
                                        if ui.memory(|memory| memory.focused())
                                            == Some(Id::new(("format_name", id)))
                                        {
                                            self.selected_stick = parent;
                                        }
                                    });
                                });
                                row.col(|ui| {
                                    changed |= cell(ui, ("quantity", id), &mut format.quantity);
                                    if ui.memory(|memory| memory.focused())
                                        == Some(Id::new(("quantity", id)))
                                    {
                                        self.selected_stick = parent;
                                    }
                                });
                                for _ in 0..4 {
                                    row.col(|_ui| {});
                                }
                                let response = row.response();
                                if response.clicked() {
                                    self.selected_stick = parent;
                                }
                                response.context_menu(|ui| {
                                    if ui.button("Rimuovi formato").clicked() {
                                        remove_format = Some(id);
                                        ui.close();
                                    }
                                });
                            }
                        });
                    });
            });
        if let Some(id) = remove_format {
            self.study.formats.retain(|format| format.id != id);
            changed = true;
        }
        ui.horizontal_wrapped(|ui| {
            if ui.button("Agg. Stick").clicked() {
                self.study.add_stick();
                self.selected_stick = self.study.sticks.last().map(|stick| stick.id);
                changed = true;
            }
            if ui
                .add_enabled(
                    !self.study.sticks.is_empty(),
                    egui::Button::new("Agg. Formato"),
                )
                .on_hover_text("Aggiunge un formato allo stick selezionato, o al primo stick")
                .clicked()
            {
                self.study.add_format();
                if let Some(parent) = self
                    .selected_stick
                    .filter(|id| self.study.sticks.iter().any(|stick| stick.id == *id))
                {
                    self.study
                        .formats
                        .last_mut()
                        .expect("just inserted")
                        .stick_id = Some(parent);
                    self.collapsed.remove(&parent);
                }
                changed = true;
            }
        });
        ui.label(
            RichText::new("Punto o virgola per i decimali · clic destro per rimuovere")
                .small()
                .color(MUTED),
        );
        changed
    }

    fn parameters_window(&mut self, ctx: &egui::Context) {
        if !self.parameters_open {
            return;
        }
        let mut open = true;
        let mut done = false;
        let mut changed = false;
        egui::Window::new("Parametri astucciatrice e ricerca")
            .id(Id::new("parameters_window"))
            .open(&mut open)
            .default_width(580.0)
            .show(ctx, |ui| {
                egui::ScrollArea::both().max_height(420.0).show(ui, |ui| {
                    changed = self.parameters(ui);
                });
                if ui.button("Chiudi").clicked() {
                    done = true;
                }
            });
        self.parameters_open = open && !done;
        if changed {
            self.invalidate_results();
        }
    }

    fn parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.heading(RichText::new("Parametri dello studio").color(ACCENT));
        ui.label(RichText::new("Geometria e vincoli applicati alla ricerca.").color(MUTED));
        ui.add_space(12.0);
        let grid_width = ui.available_width().max(550.0);
        egui::ScrollArea::horizontal()
            .id_salt("parameters_horizontal")
            .show(ui, |ui| {
                ui.set_width(grid_width);
                egui::Grid::new("parameters")
                    .num_columns(3)
                    .spacing([20.0, 15.0])
                    .show(ui, |ui| {
                        let p = &mut self.study.parameters;
                        for (label, value, unit) in [
                            ("Offset massimo stick", &mut p.offset, "mm"),
                            ("Passo massimo astucciatrice", &mut p.max_pitch, "mm"),
                            ("Incremento del passo", &mut p.pitch_step, "mm"),
                            (
                                "Limite massimo di strati",
                                &mut p.max_layers,
                                "vuoto = nessun limite",
                            ),
                            ("Rapporto A/B desiderato", &mut p.ab_target, "rapporto"),
                            ("Numero di risultati", &mut p.result_count, "risultati"),
                            ("Spessore divisore", &mut p.divider_width, "mm"),
                            ("Spessore parete cassetto", &mut p.wall_width, "mm"),
                            ("Gioco tra stick", &mut p.adjacent_clearance, "mm"),
                            ("Gioco parete/divisore", &mut p.wall_clearance, "mm"),
                            ("Extra quota B", &mut p.carton_b_extra, "mm"),
                        ] {
                            let label_response = ui.label(label);
                            changed |= ui
                                .add(
                                    TextEdit::singleline(value)
                                        .id(Id::new(("param", label)))
                                        .desired_width(100.0),
                                )
                                .labelled_by(label_response.id)
                                .changed();
                            ui.label(RichText::new(unit).small().color(MUTED));
                            ui.end_row();
                        }
                    });
            });
        ui.add_space(16.0);
        ui.heading(RichText::new("Pesi del punteggio").color(ACCENT));
        ui.label("Valori non negativi. Il filtro Pareto mantiene le sue sette metriche; i pesi ordinano le soluzioni.");
        egui::Grid::new("weights").show(ui, |ui| {
            for (label, value) in [
                "Tipi cassetto",
                "Tipi testa robot",
                "Strati",
                "Riporto",
                "Raggruppamento",
                "Stabilità",
                "Passo astucciatrice",
                "Rapporto A/B",
            ]
            .into_iter()
            .zip(&mut self.study.parameters.weights)
            {
                ui.label(label);
                changed |= ui
                    .add(
                        TextEdit::singleline(value)
                            .id(Id::new(("weight", label)))
                            .desired_width(100.0),
                    )
                    .changed();
                ui.end_row();
            }
        });
        if ui.button("Ripristina pesi incorporati").clicked() {
            self.study.parameters.weights = stick_optimizer_core::Weights::default()
                .values()
                .map(|v| v.to_string());
            changed = true;
        }
        if let Ok(ratio) = parse_decimal(&self.study.parameters.ab_target) {
            ui.label(
                RichText::new(format!("Rapporto A/B inserito: {}", format_number(ratio)))
                    .small()
                    .color(MUTED),
            );
        }
        changed
    }

    fn results_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Migliori Soluzioni").color(ACCENT));
        if let Some(message) = &self.diagnostic {
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(249, 240, 240))
                .show(ui, |ui| {
                    ui.label(message);
                });
        }
        if self.results.is_empty() {
            ui.add_space(12.0);
            ui.label("Inserisci i dati nella sidebar e premi «Calcola».");
        }
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.one_head_only, "Una sola testa")
                .on_hover_text("Mostra solo soluzioni con un tipo di testa robot; grouping 1 è escluso dal conteggio");
            egui::ComboBox::from_id_salt("result_order")
                .width(140.0)
                .selected_text(if self.order_by_pitch {
                    "Passo"
                } else {
                    "Punteggio"
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.order_by_pitch, false, "Ordina per punteggio");
                    ui.selectable_value(&mut self.order_by_pitch, true, "Ordina per passo");
                });
        });
        let rows = self.visible_results();
        if !rows.iter().any(|row| Some(row.id) == self.selected) {
            self.selected = rows.first().map(|row| row.id);
        }
        ui.label(
            RichText::new(format!(
                "{} soluzioni visibili · seleziona una riga per i dettagli",
                rows.len()
            ))
            .small()
            .color(MUTED),
        );
        let table_width = ui.available_width().max(690.0);
        egui::ScrollArea::horizontal()
            .id_salt("results_horizontal")
            .show(ui, |ui| {
                ui.set_width(table_width);
                TableBuilder::new(ui)
                    .id_salt("results")
                    .striped(true)
                    .column(Column::remainder().clip(true))
                    .columns(Column::exact(105.0), 5)
                    .max_scroll_height(175.0)
                    .sense(egui::Sense::click())
                    .header(28.0, |mut header| {
                        for label in [
                            "Posizione",
                            "Passo (mm)",
                            "Formati cassetto",
                            "Teste robot",
                            "Strati max",
                            "Punteggio",
                        ] {
                            header.col(|ui| {
                                ui.strong(label);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(38.0, rows.len(), |mut row| {
                            let result = &rows[row.index()];
                            row.set_selected(self.selected == Some(result.id));
                            row.col(|ui| {
                                ui.label(format!("Soluzione {:02}", result.id));
                            });
                            for value in [
                                format_number(result.pitch),
                                result.pockets.to_string(),
                                result.heads.to_string(),
                                result.layers.to_string(),
                                format_number(result.score),
                            ] {
                                row.col(|ui| {
                                    ui.label(value);
                                });
                            }
                            if row.response().clicked() {
                                self.selected = Some(result.id);
                            }
                        });
                    });
            });
        if let Some(result) = rows.iter().find(|row| Some(row.id) == self.selected) {
            ui.add_space(18.0);
            egui::Frame::group(ui.style())
                .fill(Color32::WHITE)
                .inner_margin(16)
                .show(ui, |ui| {
                    ui.heading(RichText::new("Dettagli Soluzione Selezionata").color(ACCENT));
                    ui.label(format!("Soluzione {:02}", result.id));
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!("Passo: {} mm", format_number(result.pitch)));
                        ui.separator();
                        ui.label(format!("{} tipi di cassetto", result.pockets));
                        ui.separator();
                        ui.label(format!(
                            "{} {}",
                            result.heads,
                            if result.heads == 1 {
                                "tipo di testa"
                            } else {
                                "tipi di testa"
                            }
                        ));
                    });
                    ui.label(format!(
                        "Punteggio: {} · {} formati",
                        format_number(result.score),
                        result.solution.candidates.len()
                    ));
                });
        } else {
            ui.add_space(18.0);
            ui.heading(RichText::new("Dettagli Soluzione Selezionata").color(ACCENT));
            ui.label("Seleziona una soluzione per visualizzare il riepilogo.");
        }
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
            for (view, label) in [
                (DetailView::Formats, "Panoramica formati"),
                (DetailView::Pockets, "Tipi cassetti usati"),
                (DetailView::Heads, "Tipi teste robot usate"),
            ] {
                ui.selectable_value(&mut self.detail_view, view, label);
            }
        });
        ui.separator();
        if let Some(result) = rows.iter().find(|r| Some(r.id) == self.selected) {
            let solution = &result.solution;
            match self.detail_view {
                DetailView::Formats => {
                    let data = solution
                        .candidates
                        .iter()
                        .map(|c| {
                            vec![
                                c.format_name.clone(),
                                c.stick_name.clone(),
                                format_number(c.input_pitch.mm()),
                                c.grouping.to_string(),
                                c.dividers.to_string(),
                                c.pockets_per_pitch.to_string(),
                                c.pocket_width.to_string(),
                                format_number(c.pocket_length),
                                c.layers.to_string(),
                                format_number(c.stack_height),
                                format_number(c.carton_a),
                                format_number(c.carton_b),
                                format_number(c.ab_ratio),
                                if c.carryover_required {
                                    format!("Sì · ciclo {}", c.carryover_cycle)
                                } else {
                                    "No".into()
                                },
                            ]
                        })
                        .collect();
                    detail_table(
                        ui,
                        "format_details",
                        &[
                            "Formato",
                            "Stick",
                            "Passo MT (mm)",
                            "Gruppo",
                            "Divisori",
                            "Cass./passo",
                            "Larg. (mm)",
                            "Lungh. (mm)",
                            "Strati",
                            "Pila (mm)",
                            "A (mm)",
                            "B (mm)",
                            "A/B",
                            "Riporto",
                        ],
                        data,
                    );
                }
                DetailView::Pockets => {
                    let data = solution
                        .pocket_types
                        .iter()
                        .enumerate()
                        .map(|(i, &(width, length, dividers, pockets))| {
                            let formats = solution
                                .candidates
                                .iter()
                                .filter(|c| {
                                    (
                                        c.pocket_width,
                                        c.pocket_length,
                                        c.dividers,
                                        c.pockets_per_pitch,
                                    ) == (width, length, dividers, pockets)
                                })
                                .map(|c| c.format_name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ");
                            vec![
                                format!("Cassetto {}", i + 1),
                                width.to_string(),
                                format_number(length),
                                dividers.to_string(),
                                pockets.to_string(),
                                formats,
                            ]
                        })
                        .collect();
                    detail_table(
                        ui,
                        "pocket_details",
                        &[
                            "Tipo",
                            "Larg. (mm)",
                            "Lungh. (mm)",
                            "Divisori",
                            "Cass./passo",
                            "Formati",
                        ],
                        data,
                    );
                }
                DetailView::Heads => {
                    ui.label("Le configurazioni con grouping 1 sono escluse dal conteggio dei tipi di testa.");
                    let data = solution
                        .head_types
                        .iter()
                        .enumerate()
                        .map(|(i, &(group, pitch))| {
                            let candidates: Vec<_> = solution
                                .candidates
                                .iter()
                                .filter(|c| (c.grouping, c.input_pitch) == (group, pitch))
                                .collect();
                            let formats = candidates
                                .iter()
                                .map(|c| c.format_name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ");
                            vec![
                                format!("Testa {}", i + 1),
                                group.to_string(),
                                format_number(pitch.mm()),
                                format_number(candidates[0].head_width),
                                formats,
                            ]
                        })
                        .collect();
                    detail_table(
                        ui,
                        "head_details",
                        &["Tipo", "Gruppo", "Passo MT (mm)", "Larg. (mm)", "Formati"],
                        data,
                    );
                }
            }
            ui.collapsing("Penalità aggregate", |ui| {
                for (label, value) in [
                    "Strati",
                    "Riporto",
                    "Raggruppamento",
                    "Stabilità",
                    "Rapporto A/B",
                ]
                .into_iter()
                .zip(solution.penalties)
                {
                    ui.label(format!("{label}: {}", format_number(value)));
                }
            });
        }
    }

    fn confirm_action(&mut self, ctx: &egui::Context) {
        let Some(action) = self.pending else {
            return;
        };
        let text = match action {
            PendingAction::EmptyStudy => {
                "Vuoi creare uno studio vuoto? I dati attuali della sessione verranno sostituiti."
                    .into()
            }
            PendingAction::ExampleStudy => {
                "Vuoi caricare l'esempio? I dati attuali della sessione verranno sostituiti.".into()
            }
            PendingAction::RemoveStick(id) => {
                let count = self
                    .study
                    .formats
                    .iter()
                    .filter(|format| format.stick_id == Some(id))
                    .count();
                format!("Rimuovere questo stick e i {count} formati associati?")
            }
        };
        let mut confirm = false;
        let response = egui::Modal::new(Id::new("confirm_action")).show(ctx, |ui| {
            ui.set_max_width(400.0);
            ui.strong("Conferma modifica");
            ui.label(text);
            ui.horizontal(|ui| {
                confirm = ui.button("Conferma").clicked();
                if ui.button("Annulla").clicked() {
                    self.pending = None;
                }
            });
        });
        if confirm {
            self.apply_pending(action);
        } else if response.should_close() {
            self.pending = None;
        }
    }
}

impl eframe::App for PrototypeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render(ui);
        #[cfg(feature = "ui-preview")]
        if self.job.is_none() {
            crate::preview::capture(ui.ctx());
        }
    }
}

fn detail_table(ui: &mut egui::Ui, id: &str, headers: &[&str], data: Vec<Vec<String>>) {
    let width = ui
        .available_width()
        .max(165.0 + (headers.len() - 1) as f32 * 111.0);
    egui::ScrollArea::horizontal().id_salt(id).show(ui, |ui| {
        ui.set_width(width);
        TableBuilder::new(ui)
            .id_salt(id)
            .striped(true)
            .column(Column::remainder().at_least(165.0).clip(true))
            .columns(Column::exact(105.0).clip(true), headers.len() - 1)
            .max_scroll_height(260.0)
            .header(28.0, |mut row| {
                for label in headers {
                    row.col(|ui| {
                        ui.strong(*label);
                    });
                }
            })
            .body(|body| {
                body.rows(30.0, data.len(), |mut row| {
                    for value in &data[row.index()] {
                        row.col(|ui| {
                            ui.label(value);
                        });
                    }
                });
            });
    });
}

fn load_diagram(ctx: &egui::Context, name: &str, bytes: &[u8]) -> Option<egui::TextureHandle> {
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    Some(ctx.load_texture(
        name,
        egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw()),
        egui::TextureOptions::LINEAR,
    ))
}

fn cell(ui: &mut egui::Ui, id: impl egui::AsId, value: &mut String) -> bool {
    ui.add_sized(
        [ui.available_width(), 26.0],
        TextEdit::singleline(value).id(Id::new(id)),
    )
    .changed()
}

fn format_number(value: f64) -> String {
    format!("{value:.6}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .replace('.', ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calculate(app: &mut PrototypeApp) {
        let report = optimize(
            &app.study.to_study().unwrap(),
            &SearchControl::default(),
            SearchLimits::default(),
        )
        .unwrap();
        app.install_report(report);
    }

    fn finish_job(app: &mut PrototypeApp) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        while app.job.is_some() {
            app.poll_search();
            assert!(
                std::time::Instant::now() < deadline,
                "worker did not finish"
            );
            std::thread::yield_now();
        }
    }

    #[test]
    fn real_results_are_blocked_by_invalid_inputs_and_cleared_after_edits() {
        let mut app = PrototypeApp::default();
        calculate(&mut app);
        assert_eq!(app.results.len(), 16);
        app.study.sticks[0].width = "non numerico".into();
        app.invalidate_results();
        assert!(app.results.is_empty());
        assert!(app.selected.is_none());
        assert!(!app.start_search());
    }

    #[test]
    fn filtering_and_sorting_preserve_stable_result_identity() {
        let mut app = PrototypeApp::default();
        calculate(&mut app);
        let expected: Vec<_> = app
            .results
            .iter()
            .filter(|r| r.heads == 1)
            .map(|r| r.id)
            .collect();
        app.one_head_only = true;
        assert_eq!(
            app.visible_results()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            expected
        );
        app.one_head_only = false;
        app.order_by_pitch = true;
        let rows = app.visible_results();
        assert!(rows.windows(2).all(|pair| pair[0].pitch <= pair[1].pitch));
        assert_eq!(rows.len(), app.results.len());
        for row in rows {
            assert_eq!(
                row.solution.as_ref(),
                app.results[row.id - 1].solution.as_ref()
            );
        }
    }

    #[test]
    fn new_study_removes_previous_results_and_inputs() {
        let mut app = PrototypeApp::default();
        calculate(&mut app);
        app.apply_pending(PendingAction::EmptyStudy);
        assert!(app.study.sticks.is_empty());
        assert!(app.results.is_empty());
        assert!(app.collapsed.is_empty());
        assert!(!app.start_search());
    }

    #[test]
    fn background_search_returns_real_results_and_input_edits_discard_stale_work() {
        let mut app = PrototypeApp::default();
        assert!(app.start_search());
        assert!(!app.start_search()); // At most one worker.
        finish_job(&mut app);
        assert_eq!(app.results.len(), 16);
        assert_eq!(app.results[0].solution.candidates.len(), 3);
        assert!(app.start_search());
        app.study.formats[0].quantity = "11".into();
        app.invalidate_results();
        finish_job(&mut app);
        assert!(app.results.is_empty());
        assert!(app.status.contains("scartato"));
    }

    #[test]
    fn cancelled_worker_never_installs_partial_or_completed_results() {
        let mut app = PrototypeApp::default();
        assert!(app.start_search());
        app.job
            .as_ref()
            .unwrap()
            .control
            .cancelled
            .store(true, Ordering::Relaxed);
        finish_job(&mut app);
        assert!(app.results.is_empty());
        assert!(app.status.contains("annullato"));
    }

    #[test]
    fn calculate_button_runs_the_worker_and_renders_real_solution_details() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        let output = draw_frame(&ctx, &mut app, vec![]);
        let pos = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) if text.galley.job.text == "Calcola" => {
                    Some(text.pos + text.galley.size() / 2.0)
                }
                _ => None,
            })
            .expect("calculate button rendered");
        for pressed in [true, false] {
            draw_frame(
                &ctx,
                &mut app,
                vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
            );
        }
        finish_job(&mut app);
        assert_eq!(app.results.len(), 16);
        let output = draw_frame(&ctx, &mut app, vec![]);
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape,
            egui::epaint::Shape::Text(text) if text.galley.job.text == "Soluzione 01")));
        for view in [DetailView::Formats, DetailView::Pockets, DetailView::Heads] {
            app.detail_view = view;
            assert!(!draw_frame(&ctx, &mut app, vec![]).shapes.is_empty());
        }
    }

    fn draw_frame(
        ctx: &egui::Context,
        app: &mut PrototypeApp,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        draw_frame_at(ctx, app, egui::vec2(1180.0, 800.0), events)
    }

    fn draw_frame_at(
        ctx: &egui::Context,
        app: &mut PrototypeApp,
        size: egui::Vec2,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                events,
                ..Default::default()
            },
            |ui| app.render(ui),
        );
        // These headless tests intentionally discard GPU texture updates.
        output.textures_delta.clear();
        output
    }

    #[test]
    fn keyboard_edits_a_table_cell_and_tab_moves_to_next_field() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        let id = app.study.sticks[0].id;
        draw_frame(&ctx, &mut app, vec![]);
        ctx.memory_mut(|memory| memory.request_focus(Id::new(("width", id))));
        draw_frame(
            &ctx,
            &mut app,
            vec![
                egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers {
                        ctrl: true,
                        command: true,
                        ..egui::Modifiers::NONE
                    },
                },
                egui::Event::Text("26,5".into()),
            ],
        );
        assert_eq!(app.study.sticks[0].width, "26,5");
        assert!(app.study.input_errors().is_empty());
        draw_frame(
            &ctx,
            &mut app,
            vec![egui::Event::Key {
                key: egui::Key::Tab,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
        assert_eq!(
            ctx.memory(|memory| memory.focused()),
            Some(Id::new(("thickness", id)))
        );
    }

    #[test]
    fn numeric_columns_remain_visible_after_zoom_change() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        ctx.set_zoom_factor(1.5);
        // Native startup may render once before knowing the window dimensions.
        draw_frame_at(&ctx, &mut app, egui::vec2(6666.7, 6666.7), vec![]);
        let size = egui::vec2(786.7, 533.3);
        let mut output = draw_frame_at(&ctx, &mut app, size, vec![]);
        for _ in 0..6 {
            output = draw_frame_at(&ctx, &mut app, size, vec![]);
        }
        for label in ["Hs", "As", "Ss", "Bs"] {
            assert!(
                output.shapes.iter().any(|shape| {
                    if let egui::epaint::Shape::Text(text) = &shape.shape {
                        text.galley.job.text == label
                            && shape.clip_rect.contains_rect(egui::Rect::from_min_size(
                                text.pos,
                                text.galley.size(),
                            ))
                    } else {
                        false
                    }
                }),
                "Column {label} clipped after zoom"
            );
        }
    }

    #[test]
    fn hierarchy_groups_children_and_collapse_keeps_the_other_stick_visible() {
        let mut app = PrototypeApp::default();
        assert!(matches!(
            app.input_rows().as_slice(),
            [
                InputRow::Stick(0),
                InputRow::Format(0),
                InputRow::Format(1),
                InputRow::Stick(1),
                InputRow::Format(2)
            ]
        ));
        app.collapsed.insert(app.study.sticks[0].id);
        assert!(matches!(
            app.input_rows().as_slice(),
            [InputRow::Stick(0), InputRow::Stick(1), InputRow::Format(2)]
        ));
        assert_eq!(app.study.formats.len(), 3);
    }

    #[test]
    fn sidebar_and_output_render_together_at_compact_size_and_increased_scale() {
        for scale in [1.0, 1.5] {
            let ctx = egui::Context::default();
            let mut app = PrototypeApp::new(&ctx);
            ctx.set_zoom_factor(scale);
            calculate(&mut app);
            draw_frame_at(&ctx, &mut app, egui::vec2(760.0, 620.0), vec![]);
            let output = draw_frame_at(&ctx, &mut app, egui::vec2(760.0, 620.0), vec![]);
            for label in [
                "Dati MT",
                "Formati",
                "Migliori Soluzioni",
                "Dettagli Soluzione Selezionata",
            ] {
                assert!(
                    output.shapes.iter().any(|shape| matches!(&shape.shape,
                    egui::epaint::Shape::Text(text) if text.galley.job.text == label)),
                    "{label} missing"
                );
            }
            let position = |label: &str| {
                output
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::epaint::Shape::Text(text) if text.galley.job.text == label => {
                            Some(text.pos)
                        }
                        _ => None,
                    })
                    .expect("heading already checked")
            };
            assert!(position("Migliori Soluzioni").x > position("Formati").x + 300.0);
            assert!(
                position("Dettagli Soluzione Selezionata").y > position("Migliori Soluzioni").y
            );
            app.parameters_open = true;
            assert!(
                !draw_frame_at(&ctx, &mut app, egui::vec2(760.0, 620.0), vec![])
                    .shapes
                    .is_empty()
            );
        }
    }

    #[test]
    fn large_tables_virtualize_rows_instead_of_rendering_every_cell() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        let small = draw_frame(&ctx, &mut app, vec![]).shapes.len();
        for _ in 0..998 {
            app.study.add_stick();
            app.study.add_format();
        }
        draw_frame(&ctx, &mut app, vec![]);
        let large = draw_frame(&ctx, &mut app, vec![]).shapes.len();
        assert!(
            large < small * 3,
            "{small} shapes for small table, {large} for large table"
        );
        assert!(app.study.input_errors().is_empty());
    }
}
