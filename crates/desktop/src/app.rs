use eframe::egui::{self, Color32, Id, RichText, TextEdit};
use egui_extras::{Column, TableBuilder};
use std::collections::HashSet;
use std::sync::{Arc, atomic::Ordering};
use std::thread::JoinHandle;
use stick_optimizer_core::{
    SearchControl, SearchError, SearchLimits, SearchReport, Solution, optimize,
};

use crate::draft::StudyDraft;
mod details;
mod files;
mod parameters;
use files::FileAction;
use parameters::ParameterEditor;
use stick_optimizer_io::{Range, ViewState};

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum InputSelection {
    Stick(u64),
    Format(u64),
}

#[derive(Clone, Copy)]
enum PendingAction {
    EmptyStudy,
    ExampleStudy,
    RemoveStick(u64),
    OpenStudy,
    CloseWindow,
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
    parameter_editor: ParameterEditor,
    parameters_draft: Option<crate::draft::Parameters>,
    parameters_error: Option<String>,
    detail_popup: Option<(usize, u64)>,
    detail_view: DetailView,
    collapsed: HashSet<u64>,
    selected_stick: Option<u64>,
    selected_input: Option<InputSelection>,
    mt_image: Option<egui::TextureHandle>,
    mt_extra_image: Option<egui::TextureHandle>,
    cartoner_image: Option<egui::TextureHandle>,
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
    current_path: Option<std::path::PathBuf>,
    dirty: bool,
    allow_close: bool,
    file_action: Option<FileAction>,
    defaults: Option<crate::draft::Parameters>,
    ranges: [Range; 5],
    filter_editor: crate::beta::FilterEditor,
    restore_view: Option<(u64, ViewState)>,
    geometry_format: Option<u64>,
}

impl Default for PrototypeApp {
    fn default() -> Self {
        Self {
            study: StudyDraft::example(),
            parameters_open: false,
            parameter_editor: ParameterEditor::default(),
            parameters_draft: None,
            parameters_error: None,
            detail_popup: None,
            detail_view: DetailView::Formats,
            collapsed: HashSet::new(),
            selected_stick: None,
            selected_input: None,
            mt_image: None,
            mt_extra_image: None,
            cartoner_image: None,
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
            current_path: None,
            dirty: false,
            allow_close: false,
            file_action: None,
            defaults: None,
            ranges: [Range::default(); 5],
            filter_editor: crate::beta::FilterEditor::default(),
            restore_view: None,
            geometry_format: None,
        }
    }
}

impl PrototypeApp {
    pub fn new(ctx: &egui::Context) -> Self {
        ctx.set_visuals(egui::Visuals::light());
        ctx.global_style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(6.0, 4.0);
            style.spacing.button_padding = egui::vec2(8.0, 4.0);
            style.spacing.window_margin = egui::Margin::same(12);
            style.spacing.menu_margin = egui::Margin::same(8);
            style.spacing.menu_width = 340.0;
            style.visuals.window_fill = Color32::WHITE;
            style.visuals.faint_bg_color = Color32::from_rgb(246, 246, 246);
            style.visuals.selection.bg_fill = ACCENT;
            style.visuals.selection.stroke.color = Color32::WHITE;
            style.visuals.widgets.active.bg_fill = ACCENT;
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(248, 233, 233);
            style.visuals.widgets.noninteractive.fg_stroke.color = MUTED;
            style.visuals.widgets.inactive.bg_fill = Color32::WHITE;
            style.visuals.widgets.inactive.bg_stroke =
                egui::Stroke::new(1.0, Color32::from_rgb(210, 212, 212));
            style.visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(2);
            style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(2);
            style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(2);
            style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(2);
            style
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(13.0));
            style
                .text_styles
                .insert(egui::TextStyle::Button, egui::FontId::proportional(13.0));
            style
                .text_styles
                .insert(egui::TextStyle::Heading, egui::FontId::proportional(17.0));
        });
        #[allow(unused_mut)] // Preview modes mutate the initial state only in QA builds.
        let mut app = Self {
            mt_image: load_diagram(
                ctx,
                "mt_diagram",
                include_bytes!("../../../img/dati_mt.jpg"),
            ),
            mt_extra_image: load_diagram(
                ctx,
                "mt_extra_diagram",
                include_bytes!("../../../img/dati_mt_menu.jpg"),
            ),
            cartoner_image: load_diagram(
                ctx,
                "cartoner_diagram",
                include_bytes!("../../../img/dati_astucciatrice.jpg"),
            ),
            stick_image: load_diagram(
                ctx,
                "stick_diagram",
                include_bytes!("../../../img/stick_dim.jpg"),
            ),
            ..Self::default()
        };
        if !cfg!(test) && std::env::var_os("STICK_OPTIMIZER_PREVIEW").is_none() {
            app.read_defaults();
        }
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
            Ok(
                "geometry" | "geometry-detail" | "detail" | "detail-compact" | "detail-scaled"
                | "detail-bottom" | "detail-carry",
            ) => {
                if std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref() == Ok("detail-scaled") {
                    ctx.set_zoom_factor(1.5);
                }
                app.start_search();
            }
            Ok("filters") => {
                app.filter_editor.edit(app.ranges);
                app.start_search();
            }
            Ok("reopened") => {
                let folder = std::path::Path::new("work/phase5");
                let path = folder.join("qa-roundtrip.stic");
                let outcome = (|| -> Result<(), stick_optimizer_io::Error> {
                    std::fs::create_dir_all(folder)?;
                    let study = app.study.to_study().map_err(|e| e.join("; "))?;
                    let report =
                        optimize(&study, &SearchControl::default(), SearchLimits::default())?;
                    app.install_report(report);
                    app.save_to(&path)?;
                    app.open_from(&path)
                })();
                if let Err(error) = outcome {
                    app.diagnostic = Some(error.to_string());
                }
            }
            Ok("unsaved") => {
                app.dirty = true;
                app.pending = Some(PendingAction::CloseWindow);
            }
            Ok("file-error") => {
                app.diagnostic = Some("Impossibile aprire lo studio: versione non supportata. Lo studio corrente è stato conservato.".into());
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
            Ok(
                mode @ ("parameters" | "parameters-weights" | "parameters-results"
                | "parameters-ab" | "parameters-mt" | "parameters-compact"
                | "parameters-scaled"),
            ) => {
                if mode == "parameters-scaled" {
                    ctx.set_zoom_factor(1.5);
                }
                app.open_parameter_editor(match mode {
                    "parameters-weights" => ParameterEditor::Weights,
                    "parameters-results" => ParameterEditor::ResultCount,
                    "parameters-ab" => ParameterEditor::AbTarget,
                    "parameters-mt" => ParameterEditor::Mt,
                    _ => ParameterEditor::Cartoner,
                });
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
        self.dirty = true;
        self.restore_view = None;
        self.geometry_format = None;
        self.detail_popup = None;
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
        let edited_view = self.restore_view.is_some().then(|| self.view_state());
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
        self.ranges = [Range::default(); 5];
        if let Some((revision, view)) = self.restore_view.take()
            && revision == self.revision
        {
            if self.dirty {
                self.apply_view(edited_view.expect("restoring view"));
            } else {
                self.apply_view(view);
            }
        }
    }

    fn apply_pending(&mut self, action: PendingAction) {
        match action {
            PendingAction::EmptyStudy => {
                self.study = StudyDraft::default();
                if let Some(parameters) = &self.defaults {
                    self.study.parameters = parameters.clone();
                }
                self.current_path = None;
                self.collapsed.clear();
                self.selected_stick = None;
            }
            PendingAction::ExampleStudy => {
                self.study = StudyDraft::example();
                self.current_path = None;
                self.collapsed.clear();
                self.selected_stick = None;
            }
            PendingAction::RemoveStick(id) => {
                self.study.remove_stick(id);
            }
            PendingAction::OpenStudy => {
                self.file_action = Some(FileAction::Open);
                self.pending = None;
                return;
            }
            PendingAction::CloseWindow => {
                self.allow_close = true;
                self.pending = None;
                return;
            }
        }
        self.selected_input = None;
        self.parameters_open = false;
        self.parameters_draft = None;
        self.filter_editor.open = false;
        self.pending = None;
        self.invalidate_results();
    }

    fn visible_results(&self) -> Vec<ResultRow> {
        let mut rows: Vec<_> = self
            .results
            .iter()
            .filter(|row| !self.one_head_only || row.heads == 1)
            .filter(|row| {
                self.ranges
                    .into_iter()
                    .zip([
                        row.pitch,
                        row.pockets as f64,
                        row.heads as f64,
                        f64::from(row.layers),
                        row.score,
                    ])
                    .all(|(range, value)| range.contains(value))
            })
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
        self.handle_file_action();
        self.shortcuts(ui.ctx());
        if self.allow_close {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if self.job.is_some() {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(50));
        }
        egui::Panel::top("menu_bar")
            .frame(egui::Frame::new().fill(Color32::WHITE).inner_margin(10))
            .show(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    let _file_menu = ui.menu_button("File", |ui| {
                        if ui
                            .add_enabled(
                                self.job.is_none(),
                                egui::Button::new("Apri studio…").shortcut_text("Ctrl+O"),
                            )
                            .clicked()
                        {
                            if self.dirty {
                                self.pending = Some(PendingAction::OpenStudy);
                            } else {
                                self.file_action = Some(FileAction::Open);
                            }
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                self.job.is_none(),
                                egui::Button::new("Salva studio").shortcut_text("Ctrl+S"),
                            )
                            .clicked()
                        {
                            self.file_action = Some(FileAction::Save);
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                self.job.is_none(),
                                egui::Button::new("Salva studio con nome…")
                                    .shortcut_text("Ctrl+Maiusc+S"),
                            )
                            .clicked()
                        {
                            self.file_action = Some(FileAction::SaveAs);
                            ui.close();
                        }
                        ui.separator();
                        if ui
                            .add(egui::Button::new("Nuovo studio").shortcut_text("Ctrl+N"))
                            .clicked()
                        {
                            self.pending = Some(PendingAction::EmptyStudy);
                            ui.close();
                        }
                        if ui.button("Carica esempio").clicked() {
                            self.pending = Some(PendingAction::ExampleStudy);
                            ui.close();
                        }
                        ui.separator();
                        ui.menu_button("Esporta CSV", |ui| {
                            if ui
                                .add_enabled(
                                    !self.visible_results().is_empty(),
                                    egui::Button::new("Soluzioni visibili…"),
                                )
                                .clicked()
                            {
                                self.file_action = Some(FileAction::Summary);
                                ui.close();
                            }
                            if ui
                                .add_enabled(
                                    self.selected.is_some(),
                                    egui::Button::new("Dettagli della soluzione…"),
                                )
                                .clicked()
                            {
                                self.file_action = Some(FileAction::Details);
                                ui.close();
                            }
                        });
                    });
                    #[cfg(feature = "ui-preview")]
                    if std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref() == Ok("file-menu") {
                        egui::Popup::open_id(
                            ui.ctx(),
                            egui::Popup::default_response_id(&_file_menu.response),
                        );
                    }
                    let _edit_menu = ui.menu_button("Modifica", |ui| {
                        for editor in ParameterEditor::ALL {
                            if ui.button(editor.title()).clicked() {
                                self.open_parameter_editor(editor);
                                ui.close();
                            }
                        }
                        ui.separator();
                        if ui.button("Salva parametri come predefiniti").clicked() {
                            self.file_action = Some(FileAction::Preferences);
                            ui.close();
                        }
                        if ui.button("Ripristina parametri incorporati").clicked() {
                            self.study.parameters = crate::draft::Parameters::default();
                            self.invalidate_results();
                            ui.close();
                        }
                    });
                    #[cfg(feature = "ui-preview")]
                    if std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref() == Ok("edit-menu") {
                        egui::Popup::open_id(
                            ui.ctx(),
                            egui::Popup::default_response_id(&_edit_menu.response),
                        );
                    }
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
                        self.dirty = true;
                    }
                    ui.label(RichText::new("Beta Rust").small().color(MUTED));
                    if self.dirty {
                        ui.label(RichText::new("• Da salvare").small().color(ACCENT));
                    }
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
                            RichText::new(
                                self.current_path
                                    .as_ref()
                                    .map(|p| format!("Studio: {}", p.display()))
                                    .unwrap_or_else(|| {
                                        "Dal menu File salva lo studio per conservare i dati."
                                            .into()
                                    }),
                            )
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
        self.details_window(ui.ctx());
        if let Some(ranges) = self.filter_editor.show(ui.ctx()) {
            self.ranges = ranges;
            self.dirty = true;
        }
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
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder().at_least(130.0).clip(true))
                    .column(Column::exact(40.0))
                    .columns(Column::exact(43.0), 4)
                    .max_scroll_height(240.0)
                    .sense(egui::Sense::CLICK)
                    .header(26.0, |mut header| {
                        for label in ["Stick / Formato", "Qtà", "Hs", "As", "Ss", "Bs"] {
                            header.col(|ui| {
                                ui.add(egui::Label::new(RichText::new(label).strong()).truncate())
                                    .on_hover_text(label);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(26.0, rows.len(), |mut row| match rows[row.index()] {
                            InputRow::Stick(index) => {
                                let stick = &mut self.study.sticks[index];
                                let id = stick.id;
                                row.set_selected(
                                    self.selected_input == Some(InputSelection::Stick(id)),
                                );
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
                                            self.selected_input = Some(InputSelection::Stick(id));
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
                                            self.selected_input = Some(InputSelection::Stick(id));
                                        }
                                    });
                                }
                                let response = row.response();
                                if response.clicked() {
                                    self.selected_stick = Some(id);
                                    self.selected_input = Some(InputSelection::Stick(id));
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
                                row.set_selected(
                                    self.selected_input == Some(InputSelection::Format(id)),
                                );
                                row.col(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.add_space(24.0);
                                        changed |= cell(ui, ("format_name", id), &mut format.name);
                                        if ui.memory(|memory| memory.focused())
                                            == Some(Id::new(("format_name", id)))
                                        {
                                            self.selected_stick = parent;
                                            self.selected_input = Some(InputSelection::Format(id));
                                        }
                                    });
                                });
                                row.col(|ui| {
                                    changed |= cell(ui, ("quantity", id), &mut format.quantity);
                                    if ui.memory(|memory| memory.focused())
                                        == Some(Id::new(("quantity", id)))
                                    {
                                        self.selected_stick = parent;
                                        self.selected_input = Some(InputSelection::Format(id));
                                    }
                                });
                                for _ in 0..4 {
                                    row.col(|_ui| {});
                                }
                                let response = row.response();
                                if response.clicked() {
                                    self.selected_stick = parent;
                                    self.selected_input = Some(InputSelection::Format(id));
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
        ui.horizontal_wrapped(|ui| {
            if ui.button("Agg. Stick").clicked() {
                self.study.add_stick();
                self.selected_stick = self.study.sticks.last().map(|stick| stick.id);
                self.selected_input = self.selected_stick.map(InputSelection::Stick);
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
                if let Some(format) = self.study.formats.last() {
                    self.selected_input = Some(InputSelection::Format(format.id));
                    if let Some(parent) = format.stick_id {
                        self.collapsed.remove(&parent);
                    }
                }
                changed = true;
            }
            if ui
                .add_enabled(self.selected_input.is_some(), egui::Button::new("Rimuovi"))
                .on_hover_text(
                    "Rimuovi la riga selezionata; per uno stick conferma anche i formati associati",
                )
                .clicked()
            {
                match self.selected_input {
                    Some(InputSelection::Stick(id)) => {
                        self.pending = Some(PendingAction::RemoveStick(id))
                    }
                    Some(InputSelection::Format(id)) => remove_format = Some(id),
                    None => {}
                }
            }
        });
        if let Some(id) = remove_format {
            self.study.formats.retain(|format| format.id != id);
            self.selected_input = None;
            changed = true;
        }
        ui.label(
            RichText::new("Punto o virgola per i decimali · clic destro per rimuovere")
                .small()
                .color(MUTED),
        );
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
            ui.add_space(8.0);
            ui.label("Inserisci i dati nella sidebar e premi «Calcola».");
        }
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            if ui.checkbox(&mut self.one_head_only, "Una sola testa")
                .on_hover_text("Mostra solo soluzioni con un tipo di testa robot; grouping 1 è escluso dal conteggio").changed() {self.dirty = true;}
            if ui.button("Filtri…").clicked() {self.filter_editor.edit(self.ranges);}
            if ui.button("Azzera filtri").clicked() {self.ranges = [Range::default();5];self.one_head_only = false;self.dirty = true;}
        });
        ui.horizontal(|ui| {
            ui.label("Ordina per:");
            egui::ComboBox::from_id_salt("result_order")
                .width(140.0)
                .selected_text(if self.order_by_pitch {
                    "Passo"
                } else {
                    "Punteggio"
                })
                .show_ui(ui, |ui| {
                    self.dirty |= ui
                        .selectable_value(&mut self.order_by_pitch, false, "Ordina per punteggio")
                        .changed();
                    self.dirty |= ui
                        .selectable_value(&mut self.order_by_pitch, true, "Ordina per passo")
                        .changed();
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
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder().clip(true))
                    .columns(Column::exact(105.0), 5)
                    .max_scroll_height(175.0)
                    .sense(egui::Sense::click())
                    .header(26.0, |mut header| {
                        for (index, label) in [
                            "Posizione",
                            "Passo (mm)",
                            "Formati cassetto",
                            "Teste robot",
                            "Strati max",
                            "Punteggio",
                        ]
                        .into_iter()
                        .enumerate()
                        {
                            header.col(|ui| {
                                table_text(ui, label, index > 0, false, true);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(24.0, rows.len(), |mut row| {
                            let result = &rows[row.index()];
                            row.set_selected(self.selected == Some(result.id));
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(format!("Soluzione {:02}", result.id)).color(
                                            if self.selected == Some(result.id) {
                                                Color32::WHITE
                                            } else {
                                                MUTED
                                            },
                                        ),
                                    )
                                    .truncate()
                                    .selectable(false),
                                );
                            });
                            for value in [
                                format_number(result.pitch),
                                result.pockets.to_string(),
                                result.heads.to_string(),
                                result.layers.to_string(),
                                format_number(result.score),
                            ] {
                                row.col(|ui| {
                                    table_text(
                                        ui,
                                        &value,
                                        true,
                                        self.selected == Some(result.id),
                                        false,
                                    );
                                });
                            }
                            if row.response().clicked() {
                                self.selected = Some(result.id);
                                self.dirty = true;
                            }
                        });
                    });
            });
        if let Some(result) = rows.iter().find(|row| Some(row.id) == self.selected) {
            ui.add_space(18.0);
            egui::Frame::group(ui.style())
                .fill(Color32::WHITE)
                .inner_margin(8)
                .show(ui, |ui| {
                    ui.strong(
                        RichText::new(format!("Soluzione {:02} · dettagli", result.id))
                            .color(ACCENT),
                    );
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
            ui.strong(RichText::new("Dettagli della soluzione").color(ACCENT));
            ui.label("Seleziona una soluzione per visualizzare il riepilogo.");
        }
        ui.add_space(8.0);
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
                    if let Some((index, open)) = detail_table(
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
                        solution
                            .candidates
                            .iter()
                            .position(|c| Some(c.format_id) == self.geometry_format),
                    ) {
                        let id = solution.candidates[index].format_id;
                        self.geometry_format = Some(id);
                        if open {
                            self.detail_popup = Some((result.id, id));
                        }
                    }
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new("Doppio clic su un formato per quote e disegno.")
                                .small()
                                .color(MUTED),
                        );
                        if ui.button("Apri dettaglio…").clicked() {
                            let id = self
                                .geometry_format
                                .filter(|id| solution.candidates.iter().any(|c| c.format_id == *id))
                                .or_else(|| solution.candidates.first().map(|c| c.format_id));
                            if let Some(id) = id {
                                self.detail_popup = Some((result.id, id));
                            }
                        }
                    });
                }
                DetailView::Pockets => {
                    let data = solution
                        .pocket_types
                        .iter()
                        .enumerate()
                        .map(|(i, pocket)| {
                            let (width, length, dividers, pockets) = (pocket.width, pocket.length, pocket.dividers, pocket.pockets_per_pitch);
                            let formats = solution
                                .candidates
                                .iter()
                                .filter(|c| c.pocket_type() == *pocket)
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
                        None,
                    );
                }
                DetailView::Heads => {
                    ui.label("Le configurazioni con grouping 1 sono escluse dal conteggio dei tipi di testa.");
                    let data = solution
                        .head_types
                        .iter()
                        .enumerate()
                        .map(|(i, head)| {
                            let candidates: Vec<_> = solution
                                .candidates
                                .iter()
                                .filter(|c| c.head_id == Some(head.id))
                                .collect();
                            let formats = candidates
                                .iter()
                                .map(|c| c.format_name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ");
                            vec![
                                format!("Testa {}", i + 1),
                                head.geometry.grouping.to_string(),
                                format_number(head.geometry.pickup_pitch.mm()),
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
                        None,
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
            PendingAction::OpenStudy => {
                "Aprire un altro studio? Le modifiche non salvate verranno perse.".into()
            }
            PendingAction::CloseWindow => {
                "Chiudere l'app? Le modifiche non salvate verranno perse.".into()
            }
        };
        let mut confirm = false;
        let response = egui::Modal::new(Id::new("confirm_action")).show(ctx, |ui| {
            ui.set_max_width(400.0);
            ui.strong("Conferma modifica");
            ui.label(text);
            ui.horizontal_wrapped(|ui| {
                if !matches!(action, PendingAction::RemoveStick(_))
                    && self.dirty
                    && ui.button("Salva e continua").clicked()
                    && self.save_current(false)
                {
                    confirm = true;
                }
                confirm |= ui
                    .button(
                        if self.dirty && !matches!(action, PendingAction::RemoveStick(_)) {
                            "Continua senza salvare"
                        } else {
                            "Conferma"
                        },
                    )
                    .clicked();
                if ui.button("Annulla").clicked() {
                    self.pending = None;
                }
            });
            if let Some(error) = &self.diagnostic {
                ui.colored_label(ACCENT, error);
            }
        });
        if confirm {
            self.apply_pending(action);
        } else if response.should_close() {
            self.pending = None;
        }
    }

    fn check_close_request(&mut self, ctx: &egui::Context) {
        let preview =
            cfg!(feature = "ui-preview") && std::env::var_os("STICK_OPTIMIZER_PREVIEW").is_some();
        if self.dirty
            && !self.allow_close
            && !preview
            && ctx.input(|i| i.viewport().close_requested())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.pending = Some(PendingAction::CloseWindow);
        }
    }
}

impl eframe::App for PrototypeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.check_close_request(ui.ctx());
        self.render(ui);
        #[cfg(feature = "ui-preview")]
        if self.job.is_none() {
            crate::preview::capture(ui.ctx());
        }
    }
}

fn detail_table(
    ui: &mut egui::Ui,
    id: &str,
    headers: &[&str],
    data: Vec<Vec<String>>,
    selected: Option<usize>,
) -> Option<(usize, bool)> {
    let mut clicked = None;
    let width = ui
        .available_width()
        .max(165.0 + (headers.len() - 1) as f32 * 111.0);
    egui::ScrollArea::horizontal().id_salt(id).show(ui, |ui| {
        ui.set_width(width);
        TableBuilder::new(ui)
            .id_salt(id)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .sense(egui::Sense::click())
            .column(Column::remainder().at_least(165.0).clip(true))
            .columns(Column::exact(105.0).clip(true), headers.len() - 1)
            .max_scroll_height(260.0)
            .header(26.0, |mut row| {
                for label in headers {
                    row.col(|ui| {
                        table_text(
                            ui,
                            label,
                            !matches!(*label, "Formato" | "Stick" | "Tipo" | "Formati" | "Riporto"),
                            false,
                            true,
                        );
                    });
                }
            })
            .body(|body| {
                body.rows(24.0, data.len(), |mut row| {
                    let is_selected = selected == Some(row.index());
                    row.set_selected(is_selected);
                    for value in &data[row.index()] {
                        row.col(|ui| {
                            table_text(
                                ui,
                                value,
                                value.replace(',', ".").parse::<f64>().is_ok(),
                                is_selected,
                                false,
                            );
                        });
                    }
                    let response = row.response();
                    if response.clicked() {
                        clicked = Some((row.index(), response.double_clicked()));
                    }
                });
            });
    });
    clicked
}

fn table_text(ui: &mut egui::Ui, value: &str, right: bool, selected: bool, strong: bool) {
    let layout = if right {
        egui::Layout::right_to_left(egui::Align::Center)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    ui.with_layout(layout, |ui| {
        let mut text = RichText::new(value).color(if selected { Color32::WHITE } else { MUTED });
        if strong {
            text = text.strong();
        }
        // Row clicks belong to the table, not to text selection inside a label.
        ui.add(egui::Label::new(text).truncate().selectable(false))
            .on_hover_text(value);
    });
}

fn load_diagram(ctx: &egui::Context, name: &str, bytes: &[u8]) -> Option<egui::TextureHandle> {
    let decoded = image::load_from_memory(bytes).ok()?;
    let limit = ctx.input(|i| i.max_texture_side) as u32;
    let image = if decoded.width() > limit || decoded.height() > limit {
        decoded.thumbnail(limit, limit)
    } else {
        decoded
    }
    .to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    Some(ctx.load_texture(
        name,
        egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw()),
        egui::TextureOptions::LINEAR,
    ))
}

fn cell(ui: &mut egui::Ui, id: impl egui::AsId, value: &mut String) -> bool {
    ui.add_sized(
        [ui.available_width(), 22.0],
        TextEdit::singleline(value).id(Id::new(id)),
    )
    .changed()
}

pub(crate) fn format_number(value: f64) -> String {
    format!("{value:.6}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .replace('.', ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_position(output: &egui::FullOutput, label: &str, minimum_x: f32) -> egui::Pos2 {
        output
            .shapes
            .iter()
            .find_map(|shape| {
                if let egui::epaint::Shape::Text(text) = &shape.shape
                    && text.galley.job.text == label
                    && text.pos.x > minimum_x
                {
                    Some(text.pos + text.galley.size() / 2.0)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Missing visible text: {label}"))
    }

    fn click_at(ctx: &egui::Context, app: &mut PrototypeApp, pos: egui::Pos2) {
        for pressed in [true, false] {
            draw_frame(
                ctx,
                app,
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
    }

    fn key_event(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    #[test]
    fn double_click_opens_the_matching_format_with_its_schematic_and_escape_closes_it() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        calculate(&mut app);
        for _ in 0..4 {
            draw_frame(&ctx, &mut app, vec![]);
        }
        let output = draw_frame(&ctx, &mut app, vec![]);
        let name = app.results[0].solution.candidates[1].format_name.clone();
        let id = app.results[0].solution.candidates[1].format_id;
        let pos = text_position(&output, &name, 400.0);
        click_at(&ctx, &mut app, pos);
        assert_eq!(app.geometry_format, Some(id));
        assert!(app.detail_popup.is_none());
        click_at(&ctx, &mut app, pos);
        assert_eq!(app.detail_popup, Some((1, id)));
        let output = draw_frame(&ctx, &mut app, vec![]);
        for text in [
            "Sezione schematica · quote in mm · contorno alto B astuccio",
            "Impilamento",
            "Passo astucciatrice",
            "Chiudi",
        ] {
            text_position(&output, text, 0.0);
        }
        draw_frame(
            &ctx,
            &mut app,
            vec![key_event(egui::Key::Escape, egui::Modifiers::NONE)],
        );
        assert!(app.detail_popup.is_none());
        let output = draw_frame(&ctx, &mut app, vec![]);
        let pos = text_position(&output, "Apri dettaglio…", 400.0);
        click_at(&ctx, &mut app, pos);
        assert_eq!(app.detail_popup, Some((1, id)));
        app.study.sticks[0].width = "27".into();
        app.invalidate_results();
        assert!(app.detail_popup.is_none());
    }

    #[test]
    fn separate_parameter_editors_validate_apply_and_cancel() {
        for editor in ParameterEditor::ALL {
            let ctx = egui::Context::default();
            let mut app = PrototypeApp::new(&ctx);
            calculate(&mut app);
            let before = app.study.parameters.clone();
            app.open_parameter_editor(editor);
            for _ in 0..4 {
                draw_frame(&ctx, &mut app, vec![]);
            }
            let output = draw_frame(&ctx, &mut app, vec![]);
            text_position(&output, editor.title(), 0.0);
            let set_value = |p: &mut crate::draft::Parameters, value: &str| match editor {
                ParameterEditor::Weights => p.weights[0] = value.into(),
                ParameterEditor::ResultCount => p.result_count = value.into(),
                ParameterEditor::AbTarget => p.ab_target = value.into(),
                ParameterEditor::Cartoner => p.pitch_step = value.into(),
                ParameterEditor::Mt => p.offset = value.into(),
            };
            set_value(app.parameters_draft.as_mut().unwrap(), "-1");
            let output = draw_frame(&ctx, &mut app, vec![]);
            click_at(&ctx, &mut app, text_position(&output, "Applica", 0.0));
            assert!(
                app.parameters_open && app.parameters_error.is_some(),
                "{editor:?}: open={}, error={:?}",
                app.parameters_open,
                app.parameters_error
            );
            assert_eq!(app.study.parameters, before);
            assert!(!app.results.is_empty());

            let value = if editor == ParameterEditor::ResultCount {
                "7"
            } else {
                "2,5"
            };
            set_value(app.parameters_draft.as_mut().unwrap(), value);
            // A popup must not overwrite changes to unrelated fields in the workspace.
            app.study.parameters.channels = "10".into();
            let mut expected = before.clone();
            expected.channels = "10".into();
            set_value(&mut expected, value);
            let output = draw_frame(&ctx, &mut app, vec![]);
            click_at(&ctx, &mut app, text_position(&output, "Applica", 0.0));
            assert!(!app.parameters_open, "{editor:?}");
            assert_eq!(app.study.parameters, expected);
            assert!(app.results.is_empty() && app.dirty);

            app.open_parameter_editor(editor);
            set_value(app.parameters_draft.as_mut().unwrap(), "99");
            let output = draw_frame(&ctx, &mut app, vec![]);
            click_at(&ctx, &mut app, text_position(&output, "Annulla", 0.0));
            assert!(!app.parameters_open);
            assert_eq!(app.study.parameters, expected);
        }
    }

    #[test]
    fn edit_menu_opens_each_python_parameter_popup() {
        for editor in ParameterEditor::ALL {
            let ctx = egui::Context::default();
            let mut app = PrototypeApp::new(&ctx);
            let output = draw_frame(&ctx, &mut app, vec![]);
            click_at(&ctx, &mut app, text_position(&output, "Modifica", 0.0));
            let output = draw_frame(&ctx, &mut app, vec![]);
            click_at(&ctx, &mut app, text_position(&output, editor.title(), 0.0));
            assert!(app.parameters_open);
            assert_eq!(app.parameter_editor, editor);
        }
    }

    #[test]
    fn parameter_dialog_cancels_without_losing_results_and_validates_before_applying() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        calculate(&mut app);
        let before = app.study.parameters.clone();
        app.dirty = false;
        app.parameters_open = true;
        draw_frame(&ctx, &mut app, vec![]);
        app.parameters_draft.as_mut().unwrap().offset = "15".into();
        draw_frame(
            &ctx,
            &mut app,
            vec![key_event(egui::Key::Escape, egui::Modifiers::NONE)],
        );
        assert!(!app.parameters_open);
        assert_eq!(app.study.parameters, before);
        assert!(!app.results.is_empty());
        assert!(!app.dirty);

        app.parameters_open = true;
        draw_frame(&ctx, &mut app, vec![]);
        app.parameters_draft.as_mut().unwrap().pitch_step = "0".into();
        let output = draw_frame(&ctx, &mut app, vec![]);
        click_at(&ctx, &mut app, text_position(&output, "Applica", 0.0));
        assert!(app.parameters_open && app.parameters_error.is_some());
        assert_eq!(app.study.parameters, before);
        assert!(!app.results.is_empty());

        app.parameters_draft.as_mut().unwrap().pitch_step = "2,5".into();
        let output = draw_frame(&ctx, &mut app, vec![]);
        click_at(&ctx, &mut app, text_position(&output, "Applica", 0.0));
        assert!(!app.parameters_open);
        assert_eq!(app.study.parameters.pitch_step, "2,5");
        assert!(app.results.is_empty() && app.dirty);
    }

    #[test]
    fn explicit_remove_distinguishes_a_format_from_its_parent_stick() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        let id = app.study.formats[1].id;
        app.selected_input = Some(InputSelection::Format(id));
        let output = draw_frame(&ctx, &mut app, vec![]);
        click_at(&ctx, &mut app, text_position(&output, "Rimuovi", 0.0));
        assert_eq!(app.study.sticks.len(), 2);
        assert_eq!(app.study.formats.len(), 2);
        assert!(!app.study.formats.iter().any(|f| f.id == id));
        app.selected_input = Some(InputSelection::Stick(app.study.sticks[0].id));
        let output = draw_frame(&ctx, &mut app, vec![]);
        click_at(&ctx, &mut app, text_position(&output, "Rimuovi", 0.0));
        assert!(matches!(app.pending, Some(PendingAction::RemoveStick(_))));
        let output = draw_frame(&ctx, &mut app, vec![]);
        click_at(&ctx, &mut app, text_position(&output, "Annulla", 0.0));
        assert_eq!(app.study.sticks.len(), 2);
        assert_eq!(app.study.formats.len(), 2);
    }

    #[test]
    fn file_shortcuts_respect_unsaved_changes_and_open_dialogs() {
        for (key, modifiers, action) in [
            (egui::Key::S, egui::Modifiers::CTRL, FileAction::Save),
            (
                egui::Key::S,
                egui::Modifiers::CTRL | egui::Modifiers::SHIFT,
                FileAction::SaveAs,
            ),
            (egui::Key::O, egui::Modifiers::CTRL, FileAction::Open),
        ] {
            let ctx = egui::Context::default();
            let mut app = PrototypeApp::default();
            let mut output = ctx.run_ui(
                egui::RawInput {
                    events: vec![key_event(key, modifiers)],
                    ..Default::default()
                },
                |ui| app.shortcuts(ui.ctx()),
            );
            output.textures_delta.clear();
            assert_eq!(
                std::mem::discriminant(&app.file_action.unwrap()),
                std::mem::discriminant(&action)
            );
        }
        for blocked in [false, true] {
            let ctx = egui::Context::default();
            let mut app = PrototypeApp {
                dirty: true,
                parameters_open: blocked,
                ..Default::default()
            };
            let mut output = ctx.run_ui(
                egui::RawInput {
                    events: vec![key_event(egui::Key::O, egui::Modifiers::CTRL)],
                    ..Default::default()
                },
                |ui| app.shortcuts(ui.ctx()),
            );
            output.textures_delta.clear();
            assert_eq!(app.pending.is_some(), !blocked);
            assert!(app.file_action.is_none());
        }
    }

    #[test]
    fn clicking_result_text_selects_the_row_and_renaming_keeps_its_calculation() {
        let ctx = egui::Context::default();
        let mut app = PrototypeApp::new(&ctx);
        calculate(&mut app);
        for _ in 0..4 {
            draw_frame(&ctx, &mut app, vec![]);
        }
        let output = draw_frame(&ctx, &mut app, vec![]);
        click_at(
            &ctx,
            &mut app,
            text_position(&output, "Soluzione 02", 400.0),
        );
        assert_eq!(app.selected, Some(2));
        app.dirty = false;
        ctx.memory_mut(|m| m.request_focus(Id::new("study_name")));
        draw_frame(
            &ctx,
            &mut app,
            vec![
                key_event(
                    egui::Key::A,
                    egui::Modifiers {
                        ctrl: true,
                        command: true,
                        ..egui::Modifiers::NONE
                    },
                ),
                egui::Event::Text("Studio rinominato".into()),
            ],
        );
        assert_eq!(app.study.name, "Studio rinominato");
        assert!(!app.results.is_empty());
        assert_eq!(app.selected, Some(2));
        assert!(app.dirty);
    }

    #[test]
    fn native_close_request_requires_a_decision_only_for_unsaved_changes() {
        for (dirty, allow_close, should_cancel) in [
            (true, false, true),
            (false, false, false),
            (true, true, false),
        ] {
            let ctx = egui::Context::default();
            let mut app = PrototypeApp {
                dirty,
                allow_close,
                ..Default::default()
            };
            let mut input = egui::RawInput::default();
            input
                .viewports
                .get_mut(&egui::ViewportId::ROOT)
                .unwrap()
                .events
                .push(egui::ViewportEvent::Close);
            let mut output = ctx.run_ui(input, |ui| app.check_close_request(ui.ctx()));
            output.textures_delta.clear();
            assert_eq!(
                output
                    .viewport_output
                    .get(&egui::ViewportId::ROOT)
                    .unwrap()
                    .commands
                    .iter()
                    .any(|cmd| matches!(cmd, egui::ViewportCommand::CancelClose)),
                should_cancel
            );
            assert_eq!(
                matches!(app.pending, Some(PendingAction::CloseWindow)),
                should_cancel
            );
        }
    }

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
        assert!(!app.results.is_empty());
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
        assert!(!app.results.is_empty());
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
        assert!(!app.results.is_empty());
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
                "Soluzione 01 · dettagli",
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
            assert!(position("Soluzione 01 · dettagli").y > position("Migliori Soluzioni").y);
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
