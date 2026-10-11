use eframe::egui::{self, Color32, Id, RichText, TextEdit};
use egui_extras::{Column, TableBuilder};

use crate::draft::{StudyDraft, parse_decimal};

const ACCENT: Color32 = Color32::from_rgb(24, 103, 99);
const MUTED: Color32 = Color32::from_rgb(91, 106, 118);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Inputs,
    Parameters,
    Results,
}

#[derive(Clone, Copy)]
enum PendingAction {
    EmptyStudy,
    ExampleStudy,
    RemoveStick(u64),
}

#[derive(Clone, Debug)]
struct DemoResult {
    id: u8,
    pitch: f64,
    pockets: u32,
    heads: u32,
    layers: u32,
    score: f64,
}

pub struct PrototypeApp {
    study: StudyDraft,
    screen: Screen,
    results: Vec<DemoResult>,
    selected: Option<u8>,
    one_head_only: bool,
    order_by_pitch: bool,
    pending: Option<PendingAction>,
    status: String,
}

impl Default for PrototypeApp {
    fn default() -> Self {
        Self {
            study: StudyDraft::example(),
            screen: Screen::Inputs,
            results: Vec::new(),
            selected: None,
            one_head_only: false,
            order_by_pitch: false,
            pending: None,
            status: "Dati di esempio pronti. Puoi modificare le celle direttamente.".into(),
        }
    }
}

impl PrototypeApp {
    pub fn new(ctx: &egui::Context) -> Self {
        ctx.set_visuals(egui::Visuals::light());
        ctx.global_style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(10.0, 9.0);
            style.spacing.button_padding = egui::vec2(12.0, 7.0);
            style.visuals.selection.bg_fill = ACCENT;
            style.visuals.selection.stroke.color = Color32::WHITE;
            style
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(15.0));
            style
                .text_styles
                .insert(egui::TextStyle::Button, egui::FontId::proportional(15.0));
            style
                .text_styles
                .insert(egui::TextStyle::Heading, egui::FontId::proportional(25.0));
        });
        #[allow(unused_mut)]
        let mut app = Self::default();
        #[cfg(feature = "ui-preview")]
        match std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref() {
            Ok("results") => {
                app.show_demo();
            }
            Ok("parameters") => {
                app.screen = Screen::Parameters;
            }
            Ok("scaled") => {
                ctx.set_zoom_factor(1.5);
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
        self.status =
            "Dati modificati. Puoi visualizzare nuovamente i risultati di esempio.".into();
    }

    fn show_demo(&mut self) -> bool {
        if !self.study.input_errors().is_empty() {
            return false;
        }
        // Fixed display fixtures: no geometry, scores or feasibility are computed.
        self.results = vec![
            DemoResult {
                id: 1,
                pitch: 240.0,
                pockets: 2,
                heads: 1,
                layers: 7,
                score: 420.0,
            },
            DemoResult {
                id: 2,
                pitch: 180.0,
                pockets: 3,
                heads: 2,
                layers: 9,
                score: 520.0,
            },
            DemoResult {
                id: 3,
                pitch: 300.0,
                pockets: 1,
                heads: 1,
                layers: 5,
                score: 690.0,
            },
        ];
        self.selected = Some(1);
        self.one_head_only = false;
        self.order_by_pitch = false;
        self.screen = Screen::Results;
        self.status =
            "Tre risultati dimostrativi disponibili. Nessuna ricerca è stata eseguita.".into();
        true
    }

    fn apply_pending(&mut self, action: PendingAction) {
        match action {
            PendingAction::EmptyStudy => {
                self.study = StudyDraft::default();
                self.screen = Screen::Inputs;
            }
            PendingAction::ExampleStudy => {
                self.study = StudyDraft::example();
                self.screen = Screen::Inputs;
            }
            PendingAction::RemoveStick(id) => {
                self.study.remove_stick(id);
            }
        }
        self.pending = None;
        self.invalidate_results();
    }

    fn visible_results(&self) -> Vec<DemoResult> {
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
        egui::Panel::top("heading")
            .frame(egui::Frame::new().fill(Color32::WHITE).inner_margin(18))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.heading(RichText::new("Stick Optimizer").color(ACCENT));
                    ui.label(RichText::new("ANTEPRIMA").small().strong().color(MUTED));
                });
                ui.label(
                    RichText::new(
                        "Studio del trasferimento stick · dalla macchina MT all'astucciatrice",
                    )
                    .color(MUTED),
                );
            });
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(235, 242, 241))
                    .inner_margin(12),
            )
            .show(ui, |ui| {
                ui.label(&self.status);
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_rgb(248, 250, 251)).inner_margin(20))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Nuovo studio").clicked() { self.pending = Some(PendingAction::EmptyStudy); }
                    if ui.button("Carica esempio").clicked() { self.pending = Some(PendingAction::ExampleStudy); }
                    ui.label(RichText::new("Le modifiche restano solo in questa sessione.").small().color(MUTED));
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    let label = ui.label("Studio");
                    if ui.add(TextEdit::singleline(&mut self.study.name).id(Id::new("study_name"))
                        .desired_width(350.0)).labelled_by(label.id).changed() {
                        self.invalidate_results();
                    }
                });
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    for (screen, text) in [(Screen::Inputs, "1  Stick e formati"),
                        (Screen::Parameters, "2  Parametri"), (Screen::Results, "3  Risultati")] {
                        ui.selectable_value(&mut self.screen, screen, text);
                    }
                });
                ui.add_space(10.0);
                egui::ScrollArea::vertical().id_salt("main_content").show(ui, |ui| {
                    let changed = match self.screen {
                        Screen::Inputs => self.inputs(ui),
                        Screen::Parameters => self.parameters(ui),
                        Screen::Results => { self.results_ui(ui); false }
                    };
                    if changed { self.invalidate_results(); }
                    if self.screen != Screen::Results {
                        ui.add_space(12.0);
                        let errors = self.study.input_errors();
                        if !errors.is_empty() {
                            egui::Frame::group(ui.style()).show(ui, |ui| {
                                ui.label(RichText::new("Completa o correggi questi dati").strong().color(Color32::from_rgb(150, 64, 30)));
                                for error in errors.iter().take(8) { ui.label(error); }
                                if errors.len() > 8 { ui.label(format!("Altri {} campi da correggere.", errors.len() - 8)); }
                            });
                        }
                        ui.add_space(8.0);
                        if ui.add_enabled(errors.is_empty(), egui::Button::new(
                            RichText::new("Mostra risultati di esempio").color(Color32::WHITE)).fill(ACCENT)).clicked() {
                            self.show_demo();
                        }
                        ui.label(RichText::new("I risultati sono illustrativi: il motore di calcolo verrà collegato nel prossimo passo.")
                            .small().color(MUTED));
                    }
                });
            });
        self.confirm_action(ui.ctx());
    }

    fn inputs(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.strong(format!("Tipi di stick · {}", self.study.sticks.len()));
            if ui.button("+ Stick").clicked() {
                self.study.add_stick();
                changed = true;
            }
        });
        ui.label(
            RichText::new("Dimensioni in millimetri. Usa virgola o punto per i decimali.")
                .small()
                .color(MUTED),
        );
        let table_width = ui.available_width().max(630.0);
        egui::ScrollArea::horizontal()
            .id_salt("stick_horizontal")
            .show(ui, |ui| {
                ui.set_width(table_width);
                TableBuilder::new(ui)
                    .id_salt("sticks")
                    .striped(true)
                    .column(Column::remainder().at_least(130.0).clip(true))
                    .columns(Column::exact(85.0), 4)
                    .column(Column::exact(78.0))
                    .max_scroll_height(180.0)
                    .header(28.0, |mut header| {
                        for label in ["Nome", "Lunghezza", "Larghezza", "Spessore", "Pinna", ""] {
                            header.col(|ui| {
                                ui.strong(label);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(34.0, self.study.sticks.len(), |mut row| {
                            let stick = &mut self.study.sticks[row.index()];
                            let id = stick.id;
                            row.col(|ui| {
                                changed |= cell(ui, ("stick_name", id), &mut stick.name);
                            });
                            for (key, value) in [
                                ("length", &mut stick.length),
                                ("width", &mut stick.width),
                                ("thickness", &mut stick.thickness),
                                ("fin", &mut stick.fin),
                            ] {
                                row.col(|ui| {
                                    changed |= cell(ui, (key, id), value);
                                });
                            }
                            row.col(|ui| {
                                if ui.small_button("Rimuovi").clicked() {
                                    self.pending = Some(PendingAction::RemoveStick(id));
                                }
                            });
                        });
                    });
            });
        ui.add_space(14.0);
        ui.horizontal(|ui| {
            ui.strong(format!("Formati · {}", self.study.formats.len()));
            if ui
                .add_enabled(
                    !self.study.sticks.is_empty(),
                    egui::Button::new("+ Formato"),
                )
                .clicked()
            {
                self.study.add_format();
                changed = true;
            }
        });
        let sticks: Vec<_> = self
            .study
            .sticks
            .iter()
            .map(|stick| (stick.id, stick.name.clone()))
            .collect();
        let mut remove = None;
        let table_width = ui.available_width().max(560.0);
        egui::ScrollArea::horizontal()
            .id_salt("format_horizontal")
            .show(ui, |ui| {
                ui.set_width(table_width);
                TableBuilder::new(ui)
                    .id_salt("formats")
                    .striped(true)
                    .column(Column::remainder().at_least(130.0).clip(true))
                    .column(Column::initial(170.0).at_least(100.0))
                    .column(Column::exact(120.0))
                    .column(Column::exact(78.0))
                    .max_scroll_height(180.0)
                    .header(28.0, |mut header| {
                        for label in ["Nome formato", "Tipo di stick", "Stick per cassetto", ""] {
                            header.col(|ui| {
                                ui.strong(label);
                            });
                        }
                    })
                    .body(|body| {
                        body.rows(34.0, self.study.formats.len(), |mut row| {
                            let format = &mut self.study.formats[row.index()];
                            let id = format.id;
                            row.col(|ui| {
                                changed |= cell(ui, ("format_name", id), &mut format.name);
                            });
                            row.col(|ui| {
                                let selected = sticks
                                    .iter()
                                    .find(|(id, _)| Some(*id) == format.stick_id)
                                    .map_or("Seleziona stick", |(_, name)| name.as_str());
                                egui::ComboBox::from_id_salt(("stick_choice", id))
                                    .selected_text(selected)
                                    .width(140.0)
                                    .show_ui(ui, |ui| {
                                        for (id, name) in &sticks {
                                            changed |= ui
                                                .selectable_value(
                                                    &mut format.stick_id,
                                                    Some(*id),
                                                    name,
                                                )
                                                .changed();
                                        }
                                    });
                            });
                            row.col(|ui| {
                                changed |= cell(ui, ("quantity", id), &mut format.quantity);
                            });
                            row.col(|ui| {
                                if ui.small_button("Rimuovi").clicked() {
                                    remove = Some(id);
                                }
                            });
                        });
                    });
            });
        if let Some(id) = remove {
            self.study.formats.retain(|format| format.id != id);
            changed = true;
        }
        changed
    }

    fn parameters(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.heading("Parametri dello studio");
        ui.label(
            RichText::new("Prima prova dei campi macchina e delle preferenze di ricerca.")
                .color(MUTED),
        );
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
                            ("Canali MT", &mut p.channels, "canali"),
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
        ui.label("In questa anteprima i parametri non producono calcoli geometrici.");
        // Parse only for a helpful local preview of decimal input, never for scoring.
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
        ui.heading("Risultati di esempio");
        egui::Frame::group(ui.style()).fill(Color32::from_rgb(255, 246, 226)).show(ui, |ui| {
            ui.label("Valori dimostrativi, indipendenti dai parametri. Non sono configurazioni calcolate o verificate.");
        });
        if self.results.is_empty() {
            ui.add_space(25.0);
            ui.strong("Nessun risultato da mostrare");
            ui.label("Completa i dati e premi «Mostra risultati di esempio» nella scheda Stick e formati.");
            if ui.button("Vai ai dati dello studio").clicked() {
                self.screen = Screen::Inputs;
            }
            return;
        }
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.one_head_only, "Mostra solo esempi con una testa");
            egui::ComboBox::from_id_salt("result_order")
                .selected_text(if self.order_by_pitch {
                    "Ordina per passo"
                } else {
                    "Ordina per punteggio"
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
                "{} esempi visibili · seleziona una riga per i dettagli",
                rows.len()
            ))
            .small()
            .color(MUTED),
        );
        let table_width = ui.available_width().max(630.0);
        egui::ScrollArea::horizontal()
            .id_salt("results_horizontal")
            .show(ui, |ui| {
                ui.set_width(table_width);
                TableBuilder::new(ui)
                    .id_salt("results")
                    .striped(true)
                    .column(Column::remainder().clip(true))
                    .columns(Column::exact(92.0), 5)
                    .sense(egui::Sense::click())
                    .header(28.0, |mut header| {
                        for label in [
                            "Esempio",
                            "Passo (mm)",
                            "Tipi cassetto",
                            "Tipi testa",
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
                                ui.label(format!("Esempio {:02}", result.id));
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
            egui::Frame::group(ui.style()).fill(Color32::WHITE).inner_margin(16).show(ui, |ui| {
                ui.strong(format!("Dettaglio · Esempio {:02}", result.id));
                ui.horizontal_wrapped(|ui| {
                    ui.label(format!("Passo: {} mm", format_number(result.pitch)));
                    ui.separator();
                    ui.label(format!("{} tipi di cassetto", result.pockets));
                    ui.separator();
                    ui.label(format!("{} {}", result.heads, if result.heads == 1 { "tipo di testa" } else { "tipi di testa" }));
                });
                ui.label(RichText::new("Scheda illustrativa. Le metriche reali arriveranno con il motore Rust.").small().color(MUTED));
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
        crate::preview::capture(ui.ctx());
    }
}

fn cell(ui: &mut egui::Ui, id: impl egui::AsId, value: &mut String) -> bool {
    ui.add_sized(
        [ui.available_width(), 26.0],
        TextEdit::singleline(value).id(Id::new(id)),
    )
    .changed()
}

fn format_number(value: f64) -> String {
    format!("{value:.2}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .replace('.', ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_is_blocked_by_invalid_inputs_and_cleared_after_edits() {
        let mut app = PrototypeApp::default();
        assert!(app.show_demo());
        assert_eq!(app.results.len(), 3);
        app.study.sticks[0].width = "non numerico".into();
        app.invalidate_results();
        assert!(app.results.is_empty());
        assert!(app.selected.is_none());
        assert!(!app.show_demo());
    }

    #[test]
    fn filtering_and_sorting_preserve_stable_result_identity() {
        let mut app = PrototypeApp::default();
        app.show_demo();
        app.one_head_only = true;
        assert_eq!(
            app.visible_results()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
        app.one_head_only = false;
        app.order_by_pitch = true;
        assert_eq!(
            app.visible_results()
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>(),
            vec![2, 1, 3]
        );
    }

    #[test]
    fn new_study_removes_previous_demo_and_inputs() {
        let mut app = PrototypeApp::default();
        app.show_demo();
        app.apply_pending(PendingAction::EmptyStudy);
        assert!(app.study.sticks.is_empty());
        assert!(app.results.is_empty());
        assert_eq!(app.screen as u8, Screen::Inputs as u8);
        assert!(!app.show_demo());
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
        for label in ["Lunghezza", "Larghezza", "Spessore", "Pinna"] {
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
    fn all_screens_render_at_compact_size_and_increased_scale() {
        for scale in [1.0, 1.5] {
            let ctx = egui::Context::default();
            let mut app = PrototypeApp::new(&ctx);
            ctx.set_zoom_factor(scale);
            app.show_demo();
            for screen in [Screen::Inputs, Screen::Parameters, Screen::Results] {
                app.screen = screen;
                let mut output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(760.0, 620.0),
                        )),
                        ..Default::default()
                    },
                    |ui| app.render(ui),
                );
                output.textures_delta.clear();
                assert!(!output.shapes.is_empty());
            }
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
