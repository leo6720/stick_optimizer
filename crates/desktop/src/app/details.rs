//! Full format detail: core values and its schematic share one window.
use super::*;
use stick_optimizer_core::Candidate;

impl PrototypeApp {
    pub(super) fn shortcuts(&mut self, ctx: &egui::Context) {
        // File commands must not bypass an open decision or dialog.
        if self.pending.is_some()
            || self.parameters_open
            || self.filter_editor.open
            || self.detail_popup.is_some()
        {
            return;
        }
        let pressed = |modifiers, key| {
            ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(modifiers, key)))
        };
        if self.job.is_none() {
            if pressed(egui::Modifiers::CTRL | egui::Modifiers::SHIFT, egui::Key::S) {
                self.file_action = Some(FileAction::SaveAs);
            } else if pressed(egui::Modifiers::CTRL, egui::Key::S) {
                self.file_action = Some(FileAction::Save);
            } else if pressed(egui::Modifiers::CTRL, egui::Key::O) {
                if self.dirty {
                    self.pending = Some(PendingAction::OpenStudy);
                } else {
                    self.file_action = Some(FileAction::Open);
                }
            }
        }
        if pressed(egui::Modifiers::CTRL, egui::Key::N) {
            self.pending = Some(PendingAction::EmptyStudy);
        }
    }

    pub(super) fn details_window(&mut self, ctx: &egui::Context) {
        #[cfg(feature = "ui-preview")]
        if self.detail_popup.is_none()
            && self.job.is_none()
            && matches!(
                std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref(),
                Ok("geometry"
                    | "geometry-detail"
                    | "detail"
                    | "detail-compact"
                    | "detail-scaled"
                    | "detail-bottom"
                    | "detail-carry")
            )
            && let Some(result) = self.results.first()
            && let Some(c) = result.solution.candidates.get(
                if std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref() == Ok("detail-carry") {
                    1
                } else {
                    0
                },
            )
        {
            self.detail_popup = Some((result.id, c.format_id));
        }
        let Some((rank, mut format_id)) = self.detail_popup else {
            return;
        };
        let Some(result) = self.results.iter().find(|r| r.id == rank) else {
            self.detail_popup = None;
            return;
        };
        let solution = Arc::clone(&result.solution);
        let mut open = true;
        let mut close = false;
        let available = ctx.content_rect();
        let width = (available.width() - 48.0).clamp(300.0, 690.0);
        let height = (available.height() - 64.0).clamp(280.0, 720.0);
        egui::Window::new(format!("Dettaglio formato · Soluzione {rank:02}"))
            .id(Id::new("format_detail_window"))
            .open(&mut open)
            .collapsible(false)
            .default_width(width)
            .default_height(height)
            .min_size(egui::vec2(300.0, 240.0))
            .default_pos(available.center() - egui::vec2(width / 2.0, (available.height() - 48.0).min(720.0) / 2.0))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Formato:");
                    egui::ComboBox::from_id_salt("popup_format")
                        .selected_text(solution.candidates.iter().find(|c| c.format_id == format_id).map(|c| c.format_name.as_str()).unwrap_or("Formato"))
                        .show_ui(ui, |ui| {
                            for c in &solution.candidates { ui.selectable_value(&mut format_id, c.format_id, &c.format_name); }
                        });
                });
                egui::ScrollArea::vertical()
                    .id_salt("format_popup_scroll")
                    .max_height((ui.available_height() - 42.0).max(100.0))
                    .show(ui, |ui| {
                        if let Some(c) = solution.candidates.iter().find(|c| c.format_id == format_id) {
                            ui.strong(format!("{} · {}", c.format_name, c.stick_name));
                            ui.label(c.head_id.map(|id|format!("Testa {} · ID {id}", id+1)).unwrap_or_else(||"Nessuna testa · gruppo 1".into()));
                            if let Ok(study) = self.study.to_study()
                                && let Some(g) = stick_optimizer_core::schematic(&study, c)
                            { crate::beta::draw_geometry(ui, &g); }
                            if c.carryover_required {
                                ui.label(format!("Riporto richiesto · ciclo {}. Ultimo strato parziale; il ciclo non è animato.", c.carryover_cycle));
                            }
                            ui.separator();
                            candidate_values(ui, c);
                            if let Ok(study)=self.study.to_study() {
                                configuration_values(ui,&study,&solution,c);
                            }
                            #[cfg(feature = "ui-preview")]
                            if std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref() == Ok("detail-bottom") {
                                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                            }
                        }
                    });
                ui.separator();
                ui.horizontal(|ui| {
                    close = ui.button("Chiudi").clicked();
                    ui.label(RichText::new("Esc per chiudere").small().color(MUTED));
                });
                close |= ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
            });
        self.detail_popup = (open && !close).then_some((rank, format_id));
    }
}

fn candidate_values(ui: &mut egui::Ui, c: &Candidate) {
    let mm = |v| format!("{} mm", format_number(v));
    let sections = [
        (
            "Impilamento",
            vec![
                ("Raggruppamento", c.grouping.to_string()),
                ("Strati", c.layers.to_string()),
                ("Altezza pila", mm(c.stack_height)),
                (
                    "Riporto richiesto",
                    if c.carryover_required { "Sì" } else { "No" }.into(),
                ),
                ("Ciclo riporto (passi)", c.carryover_cycle.to_string()),
            ],
        ),
        (
            "Cassetto e trasporto",
            vec![
                ("Larghezza cassetto", format!("{} mm", c.pocket_width)),
                ("Lunghezza cassetto", mm(c.pocket_length)),
                ("Divisori", c.dividers.to_string()),
                ("Cassetti per passo", c.pockets_per_pitch.to_string()),
                ("Passo cassetto", mm(c.pocket_pitch)),
                ("Passo astucciatrice", mm(c.cartoner_pitch.mm())),
                ("Passo MT", mm(c.input_pitch.mm())),
                ("Larghezza occupata", mm(c.occupied_width)),
                ("Spazio residuo", mm(c.unused_space)),
            ],
        ),
        (
            "Astuccio e testa robot",
            vec![
                ("Quota A", mm(c.carton_a)),
                ("Quota B", mm(c.carton_b)),
                ("Quota H", mm(c.pocket_length)),
                ("Rapporto A/B", format_number(c.ab_ratio)),
                ("Depositi", c.deposits.to_string()),
                ("Larghezza testa", mm(c.head_width)),
            ],
        ),
        (
            "Stabilità e penalità",
            vec![
                ("Larghezza non supportata", mm(c.unsupported_width)),
                ("Rapporto larghezza", format_number(c.width_ratio)),
                ("Penalità strati", format_number(c.penalties[0])),
                ("Penalità riporto", format_number(c.penalties[1])),
                ("Penalità raggruppamento", format_number(c.penalties[2])),
                ("Penalità stabilità", format_number(c.penalties[3])),
                ("Penalità A/B", format_number(c.penalties[4])),
            ],
        ),
    ];
    // One grid keeps the value column aligned across all sections.
    egui::Grid::new("candidate_values")
        .num_columns(2)
        .spacing([24.0, 4.0])
        .show(ui, |ui| {
            for (title, values) in sections {
                ui.strong(RichText::new(title).color(ACCENT));
                ui.label("");
                ui.end_row();
                for (label, value) in values {
                    ui.label(label);
                    ui.label(value);
                    ui.end_row();
                }
            }
        });
}

fn configuration_values(
    ui: &mut egui::Ui,
    study: &stick_optimizer_core::Study,
    solution: &stick_optimizer_core::Solution,
    c: &Candidate,
) {
    let Some(d) = stick_optimizer_core::candidate_dimensions(study, c) else {
        ui.colored_label(ACCENT, "Quote della configurazione non disponibili.");
        return;
    };
    ui.separator();
    ui.strong(RichText::new("Configurazione effettiva e giochi").color(ACCENT));
    if let Some(head) = c
        .head_id
        .and_then(|id| solution.head_types.iter().find(|h| h.id == id))
    {
        ui.label(if head.geometry.format_ids.len() > 1 {
            "Testa condivisa"
        } else {
            "Testa separata"
        });
        let formats = solution
            .candidates
            .iter()
            .filter(|c| c.head_id == Some(head.id))
            .map(|c| format!("{} (ID {})", c.format_name, c.format_id))
            .collect::<Vec<_>>()
            .join(", ");
        ui.label(format!("Formati assegnati: {formats}"));
    }
    egui::Grid::new("configuration_values")
        .num_columns(2)
        .spacing([24.0, 4.0])
        .show(ui, |ui| {
            let mut values = vec![
                (
                    "Limite interasse aggiuntivo",
                    study.settings.max_extra_spacing,
                ),
                ("Spessore divisore", study.settings.divider_width),
                (
                    "Gioco minimo stick-parete/divisore",
                    study.settings.wall_clearance,
                ),
                (
                    "Gioco minimo stick-stick",
                    study.settings.adjacent_clearance,
                ),
                ("Gioco effettivo parete sinistra", d.left_wall_clearance),
                ("Gioco effettivo parete destra", d.right_wall_clearance),
            ];
            if let (Some(spacing), Some(minimum), Some(extra)) = (
                d.approach_spacing,
                d.minimum_approach_spacing,
                d.extra_spacing,
            ) {
                values.extend([
                    ("Interasse finale di avvicinamento", spacing),
                    ("Interasse minimo del formato", minimum),
                    ("Interasse aggiuntivo del formato", extra),
                ]);
            }
            for (label, value) in values {
                ui.label(label);
                ui.label(format!("{} mm", format_number(value)));
                ui.end_row();
            }
        });
    ui.label("Coordinate locali del primo strato, dall'interno della parete sinistra:");
    ui.label(format!(
        "Centri stick (mm): {}",
        c.geometry
            .stick_centres
            .iter()
            .map(|&v| format_number(v))
            .collect::<Vec<_>>()
            .join("; ")
    ));
    ui.label(format!(
        "Bordi sinistri divisori (mm): {}",
        c.geometry
            .divider_lefts
            .iter()
            .map(|&v| format_number(v))
            .collect::<Vec<_>>()
            .join("; ")
    ));
    for (i, gap) in d.gaps.iter().enumerate() {
        ui.label(format!(
            "Distanza {}–{}: {} mm · gioco stick-stick: {} mm{}",
            i + 1,
            i + 2,
            format_number(gap.centre_spacing),
            format_number(gap.stick_clearance),
            if gap.structural {
                " · strutturale (esclusa dal limite aggiuntivo)"
            } else {
                ""
            }
        ));
        if let Some([left, right]) = gap.divider_clearances {
            ui.label(format!(
                "Giochi ai lati del divisore: {} / {} mm",
                format_number(left),
                format_number(right)
            ));
        }
    }
}
