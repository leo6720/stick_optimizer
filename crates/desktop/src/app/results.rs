//! Output composition only: values and identities remain supplied by core results.
use super::*;
use egui_extras::{Column, TableBuilder};

impl PrototypeApp {
    pub(super) fn results_header(&mut self, ui: &mut egui::Ui) {
        let wide = ui.available_width() >= 520.0;
        let title = |ui: &mut egui::Ui| {
            crate::ui::section_heading(
                ui,
                "Migliori Soluzioni",
                "Confronta le configurazioni e consulta i dettagli della selezione.",
            );
        };
        if wide {
            ui.horizontal(|ui| {
                let width = ui.available_width() - 120.0;
                ui.allocate_ui_with_layout(
                    egui::vec2(width, 44.0),
                    egui::Layout::top_down(egui::Align::LEFT),
                    title,
                );
                self.calculate_button(ui);
            });
        } else {
            title(ui);
            ui.add_space(4.0);
            self.calculate_button(ui);
        }
    }

    fn calculate_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_enabled(
                self.job.is_none() && self.study.input_errors().is_empty(),
                crate::ui::button(ui, "Calcola", crate::ui::ButtonVariant::Primary)
                    .min_size(egui::vec2(100.0, 38.0)),
            )
            .clicked()
            && self.job.is_none()
        {
            self.start_search();
        }
    }

    pub(super) fn results_ui(&mut self, ui: &mut egui::Ui) {
        if let Some(message) = &self.diagnostic {
            egui::Frame::group(ui.style())
                .fill(crate::ui::tokens(ui).hover)
                .show(ui, |ui| {
                    ui.label(message);
                });
        }
        if self.results.is_empty() {
            ui.add_space(8.0);
            ui.label("Inserisci i dati nella sidebar e premi «Calcola».");
        }
        ui.add_space(8.0);
        ui.scope(|ui| {
            ui.spacing_mut().interact_size.y = 32.0;
            ui.horizontal_wrapped(|ui| {
            if ui.checkbox(&mut self.one_head_only, "Una sola testa")
                .on_hover_text("Mostra solo soluzioni con un tipo di testa robot; grouping 1 è escluso dal conteggio").changed() {self.dirty = true;}
            if ui.add(crate::ui::button(ui, "Filtri…", crate::ui::ButtonVariant::Secondary).min_size(egui::vec2(72.0,32.0))).clicked() {self.filter_editor.edit(self.ranges);}
            if ui.add(crate::ui::button(ui, "Azzera filtri", crate::ui::ButtonVariant::Quiet)).clicked() {self.ranges = [Range::default();5];self.one_head_only = false;self.dirty = true;}
            ui.separator();
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
            });
        let rows = self.visible_results();
        if !rows.iter().any(|row| Some(row.id) == self.selected) {
            self.selected = rows.first().map(|row| row.id);
        }
        ui.add_space(6.0);
        crate::ui::data_surface(ui, |ui| {
            let table_width = ui.available_width().max(740.0);
            egui::ScrollArea::horizontal()
                .id_salt("results_horizontal")
                .show(ui, |ui| {
                    ui.set_width(table_width);
                    TableBuilder::new(ui)
                        .id_salt("results")
                        .striped(true)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::remainder().at_least(155.0).clip(true))
                        .columns(Column::remainder().at_least(105.0).clip(true), 5)
                        .min_scrolled_height(0.0)
                        .max_scroll_height(160.0)
                        .sense(egui::Sense::click())
                        .header(32.0, |mut header| {
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
                                    crate::ui::data_heading(ui, label, index > 0);
                                });
                            }
                        })
                        .body(|body| {
                            body.rows(32.0, rows.len(), |mut row| {
                                let result = &rows[row.index()];
                                row.set_selected(self.selected == Some(result.id));
                                row.col(|ui| {
                                    let selected = self.selected == Some(result.id);
                                    ui.horizontal(|ui| {
                                        ui.label(if selected { "✓" } else { "  " });
                                        table_text(
                                            ui,
                                            &format!("Soluzione {:02}", result.id),
                                            false,
                                            selected,
                                            selected,
                                        );
                                    });
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
        });
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{} soluzioni", rows.len()))
                        .small()
                        .color(crate::ui::tokens(ui).muted),
                );
            });
        });
        if let Some(result) = rows.iter().find(|row| Some(row.id) == self.selected) {
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new(format!("Soluzione {:02} · dettagli", result.id))
                        .size(21.0)
                        .strong(),
                );
                crate::ui::container(ui, crate::ui::Surface::Inset)
                    .corner_radius(4)
                    .inner_margin(5)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Selezionata").small().strong());
                    });
                if self.detail_view == DetailView::Formats {
                    let solution = &result.solution;
                    if ui.button("Apri dettaglio…").clicked() {
                        let id = self
                            .geometry_format
                            .filter(|id| solution.candidates.iter().any(|c| c.format_id == *id))
                            .or_else(|| solution.candidates.first().map(|c| c.format_id));
                        if let Some(id) = id {
                            self.detail_popup = Some((result.id, id));
                        }
                    }
                }
            });
            ui.add_space(6.0);
            crate::ui::container(ui, crate::ui::Surface::Inset)
                .corner_radius(5)
                .inner_margin(12)
                .show(ui, |ui| {
                    let metrics = [
                        (format!("{} mm", format_number(result.pitch)), "Passo"),
                        (result.pockets.to_string(), "tipi di cassetto"),
                        (
                            result.heads.to_string(),
                            if result.heads == 1 {
                                "tipo di testa"
                            } else {
                                "tipi di testa"
                            },
                        ),
                        (result.solution.candidates.len().to_string(), "formati"),
                    ];
                    ui.set_width(ui.available_width());
                    if ui.available_width() >= 480.0 {
                        ui.columns(4, |columns| {
                            for (column, (value, label)) in columns.iter_mut().zip(&metrics) {
                                crate::ui::metric(column, value, label);
                            }
                        });
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            for (value, label) in &metrics {
                                crate::ui::metric(ui, value, label);
                                ui.add_space(12.0);
                            }
                        });
                    }
                    ui.label(
                        RichText::new(format!("Punteggio: {}", format_number(result.score)))
                            .small()
                            .color(crate::ui::tokens(ui).muted),
                    );
                });
        } else {
            ui.add_space(18.0);
            ui.strong(
                RichText::new("Dettagli della soluzione").color(crate::ui::tokens(ui).accent),
            );
            ui.label("Seleziona una soluzione per visualizzare il riepilogo.");
        }
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 16.0;
            for (view, label) in [
                (DetailView::Formats, "Panoramica formati"),
                (DetailView::Pockets, "Tipi cassetti usati"),
                (DetailView::Heads, "Tipi teste robot usate"),
            ] {
                if crate::ui::detail_selector(ui, label, self.detail_view == view).clicked() {
                    self.detail_view = view;
                }
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
                                c.head_id
                                    .map(|id| format!("Testa {} · ID {id}", id + 1))
                                    .unwrap_or_else(|| "Nessuna · gruppo 1".into()),
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
                            "Testa assegnata",
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
                                .color(crate::ui::tokens(ui).muted),
                        );
                    });
                }
                DetailView::Pockets => {
                    let data = solution
                        .pocket_types
                        .iter()
                        .enumerate()
                        .map(|(i, pocket)| {
                            let (width, length, dividers, pockets) = (
                                pocket.width,
                                pocket.length,
                                pocket.dividers,
                                pocket.pockets_per_pitch,
                            );
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
                                pocket
                                    .divider_lefts
                                    .iter()
                                    .map(|&v| format_number(v))
                                    .collect::<Vec<_>>()
                                    .join("; "),
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
                            "Divisori x (mm)",
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
                        .map(|head| {
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
                                format!("Testa {}", head.id + 1),
                                head.id.to_string(),
                                if candidates.len() > 1 {
                                    "Condivisa"
                                } else {
                                    "Separata"
                                }
                                .into(),
                                format_number(head.geometry.approach_spacing),
                                head.geometry.grouping.to_string(),
                                format_number(head.geometry.pickup_pitch.mm()),
                                format_number(candidates[0].head_width),
                                formats,
                            ]
                        })
                        .collect();
                    if let Some((index, open)) = detail_table(
                        ui,
                        "head_details",
                        &[
                            "Tipo",
                            "ID",
                            "Assegnazione",
                            "Interasse avv. (mm)",
                            "Gruppo",
                            "Passo MT (mm)",
                            "Larg. (mm)",
                            "Formati",
                        ],
                        data,
                        self.head_popup
                            .as_ref()
                            .filter(|p| p.rank == result.id)
                            .and_then(|p| {
                                solution.head_types.iter().position(|h| h.id == p.head_id)
                            }),
                    ) && open
                    {
                        self.head_popup = heads::HeadPopup::new(
                            result.id,
                            solution,
                            solution.head_types[index].id,
                        );
                    }
                    ui.label(
                        RichText::new("Doppio clic su una testa per la vista prima/dopo.")
                            .small()
                            .color(crate::ui::tokens(ui).muted),
                    );
                }
            }
            ui.add_space(8.0);
            crate::ui::container(ui, crate::ui::Surface::Inset)
                .corner_radius(4)
                .inner_margin(8)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.collapsing("Penalità aggregate", |ui| {
                        ui.horizontal_wrapped(|ui| {
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
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(label)
                                            .small()
                                            .color(crate::ui::tokens(ui).muted),
                                    );
                                    ui.strong(format_number(value));
                                });
                                ui.add_space(12.0);
                            }
                        });
                    });
                });
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
    crate::ui::data_surface(ui, |ui| {
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
                .header(32.0, |mut row| {
                    for label in headers {
                        row.col(|ui| {
                            crate::ui::data_heading(
                                ui,
                                label,
                                !matches!(
                                    *label,
                                    "Formato" | "Stick" | "Tipo" | "Formati" | "Riporto"
                                ),
                            );
                        });
                    }
                })
                .body(|body| {
                    body.rows(30.0, data.len(), |mut row| {
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
    });
    clicked
}

fn table_text(ui: &mut egui::Ui, value: &str, right: bool, selected: bool, strong: bool) {
    crate::ui::table_label(ui, value, right, selected, strong);
}
