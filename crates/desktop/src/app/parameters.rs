use super::*;
use crate::draft::Parameters;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(super) enum ParameterEditor {
    Weights,
    ResultCount,
    AbTarget,
    #[default]
    Cartoner,
    Mt,
}

impl ParameterEditor {
    pub(super) const ALL: [Self; 5] = [
        Self::Weights,
        Self::ResultCount,
        Self::AbTarget,
        Self::Cartoner,
        Self::Mt,
    ];

    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Weights => "Pesi di calcolo",
            Self::ResultCount => "Numero di risultati",
            Self::AbTarget => "Obiettivo A/B astuccio",
            Self::Cartoner => "Dati astucciatrice",
            Self::Mt => "Dati aggiuntivi MT",
        }
    }

    // Copy only this editor's fields, preserving changes elsewhere in the workspace.
    fn copy_fields(self, source: &Parameters, target: &mut Parameters) {
        match self {
            Self::Weights => target.weights.clone_from(&source.weights),
            Self::ResultCount => target.result_count.clone_from(&source.result_count),
            Self::AbTarget => target.ab_target.clone_from(&source.ab_target),
            Self::Mt => target.offset.clone_from(&source.offset),
            Self::Cartoner => {
                target.max_pitch.clone_from(&source.max_pitch);
                target.pitch_step.clone_from(&source.pitch_step);
                target.max_layers.clone_from(&source.max_layers);
                target.divider_width.clone_from(&source.divider_width);
                target.wall_width.clone_from(&source.wall_width);
                target
                    .max_extra_spacing
                    .clone_from(&source.max_extra_spacing);
                target.wall_clearance.clone_from(&source.wall_clearance);
                target
                    .adjacent_clearance
                    .clone_from(&source.adjacent_clearance);
                target.carton_b_extra.clone_from(&source.carton_b_extra);
            }
        }
    }

    fn fields(self, p: &mut Parameters) -> Vec<(&'static str, &mut String, &'static str)> {
        match self {
            Self::ResultCount => vec![(
                "Numero di risultati",
                &mut p.result_count,
                "intero positivo",
            )],
            Self::AbTarget => vec![("Rapporto A/B desiderato", &mut p.ab_target, "rapporto")],
            Self::Mt => vec![("D - Offset max stick", &mut p.offset, "mm")],
            Self::Cartoner => vec![
                ("C_1 - Larghezza divisore", &mut p.divider_width, "mm"),
                ("C_2 - Larghezza parete", &mut p.wall_width, "mm"),
                ("C_3 - Gioco stick-parete/div", &mut p.wall_clearance, "mm"),
                ("C_4 - Gioco stick-stick", &mut p.adjacent_clearance, "mm"),
                (
                    "Interasse aggiuntivo massimo",
                    &mut p.max_extra_spacing,
                    "mm",
                ),
                ("B_extra - Quota B aggiuntiva", &mut p.carton_b_extra, "mm"),
                (
                    "P_tp - Passo max trasporto prodotto",
                    &mut p.max_pitch,
                    "mm",
                ),
                (
                    "Incremento passo trasporto prodotto",
                    &mut p.pitch_step,
                    "mm",
                ),
                ("Max strati", &mut p.max_layers, "vuoto = nessun limite"),
            ],
            Self::Weights => [
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
            .zip(&mut p.weights)
            .map(|(label, value)| (label, value, "peso"))
            .collect(),
        }
    }
}

impl PrototypeApp {
    pub(super) fn open_parameter_editor(&mut self, editor: ParameterEditor) {
        self.parameter_editor = editor;
        self.parameters_open = true;
        self.parameters_draft = Some(self.study.parameters.clone());
        self.parameters_error = None;
    }

    pub(super) fn parameters_window(&mut self, ctx: &egui::Context) {
        if !self.parameters_open {
            self.parameters_draft = None;
            self.parameters_error = None;
            return;
        }
        let editor = self.parameter_editor;
        let mut draft = self
            .parameters_draft
            .take()
            .unwrap_or_else(|| self.study.parameters.clone());
        let mut open = true;
        let mut apply = false;
        let mut cancel = false;
        egui::Window::new(editor.title())
            .id(Id::new(("parameters_window", editor)))
            .open(&mut open)
            .frame(crate::ui::dialog_frame(ctx))
            .default_width((ctx.content_rect().width() - 64.0).clamp(280.0, 620.0))
            .default_pos(egui::pos2((ctx.content_rect().width() - 652.0).max(32.0) / 2.0, 32.0))
            .collapsible(false)
            .show(ctx, |ui| {
                crate::ui::dialog_intro(ui, "Modifica i parametri del gruppo. Applica conferma; Annulla conserva lo studio.");
                egui::ScrollArea::vertical()
                    .id_salt(("parameter_body", editor))
                    .max_height(crate::ui::dialog_body_height(ctx, 260.0).min((ui.available_height() - 72.0).max(60.0)))
                    .show(ui, |ui| {
                        if let Some(error) = &self.parameters_error {
                            crate::ui::error_notice(ui, error);
                            ui.add_space(8.0);
                        }
                        let diagram = match editor {
                            ParameterEditor::Cartoner => self.cartoner_image.as_ref(),
                            ParameterEditor::Mt => self.mt_extra_image.as_ref(),
                            _ => None,
                        };
                        if let Some(image) = diagram {
                            crate::ui::dialog_section(ui, |ui| {
                                ui.add(egui::Image::new(image).max_width(ui.available_width()).max_height(180.0).shrink_to_fit());
                            });
                            ui.add_space(8.0);
                        }
                        if editor == ParameterEditor::Weights {
                            ui.label("Valori non negativi. I pesi ordinano le soluzioni; il filtro Pareto mantiene le sue sette metriche.");
                        }
                        crate::ui::dialog_section(ui, |ui| {
                            let compact = ui.available_width() < 480.0;
                            egui::Grid::new(("parameter_fields", editor))
                            .num_columns(if compact { 1 } else { 3 }).spacing([16.0, 10.0])
                            .show(ui, |ui| {
                                for (label, value, unit) in editor.fields(&mut draft) {
                                    let response = ui.label(label);
                                    if compact { ui.end_row(); }
                                    ui.horizontal(|ui| {
                                    ui.add(crate::ui::field(value)
                                        .id(Id::new(("parameter", editor, label)))
                                        .horizontal_align(egui::Align::RIGHT)
                                        .min_size(egui::vec2(100.0, 26.0))
                                        .desired_width(110.0)).labelled_by(response.id);
                                    if compact {
                                        ui.label(RichText::new(unit).small().color(crate::ui::tokens(ui).muted));
                                    }
                                    });
                                    if !compact {
                                    ui.label(RichText::new(unit).small().color(crate::ui::tokens(ui).muted));
                                    }
                                    ui.end_row();
                                }
                            });
                        });
                        if editor == ParameterEditor::AbTarget {
                            ui.label("Accetta il punto o la virgola come separatore decimale.");
                        }
                        ui.add_space(8.0);
                        if ui.add(crate::ui::button(ui, "Ripristina valori predefiniti", crate::ui::ButtonVariant::Secondary)).clicked() {
                            editor.copy_fields(self.defaults.as_ref().unwrap_or(&Parameters::default()), &mut draft);
                        }
                    });
                (cancel, apply) = crate::ui::dialog_actions(ui, "Applica");
                cancel |= ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
            });
        if apply {
            let mut merged = self.study.parameters.clone();
            editor.copy_fields(&draft, &mut merged);
            if self.apply_parameters(&merged) {
                open = false;
            }
        }
        self.parameters_open = open && !cancel;
        if self.parameters_open {
            self.parameters_draft = Some(draft);
        } else {
            self.parameters_error = None;
        }
    }

    fn apply_parameters(&mut self, draft: &Parameters) -> bool {
        let mut check = StudyDraft::example();
        check.parameters = draft.clone();
        if let Err(errors) = check.to_study() {
            self.parameters_error = Some(errors.join("; "));
            return false;
        }
        if self.study.parameters != *draft {
            self.study.parameters = draft.clone();
            self.invalidate_results();
        }
        true
    }
}
