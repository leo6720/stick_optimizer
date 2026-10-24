//! Input compositions. Editable drafts and actions keep their original identities.
use super::*;
use crate::ui::{self as design, ButtonVariant, Surface};

impl PrototypeApp {
    pub(super) fn inputs(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        let mut remove_format = None;
        let rows = self.input_rows();
        let ctx = ui.ctx().clone();
        // Keep the hierarchy virtualized: a stick has its own dimensional block,
        // followed by its format rows. Only visible blocks create widgets.
        let heights = rows.iter().map(|row| match row {
            InputRow::Stick(index) => {
                if *index == 0 {
                    184.0
                } else {
                    196.0
                }
            }
            InputRow::Format(_) => 38.0,
        });
        let height = (ui.available_height() - 74.0).clamp(180.0, 680.0);
        let row_spacing = ui.spacing().item_spacing.y;
        ui.spacing_mut().item_spacing.y = 0.0;
        TableBuilder::new(ui)
            .id_salt("hierarchical_inputs")
            .column(Column::remainder())
            .max_scroll_height(height)
            .sense(egui::Sense::click())
            .body(|body| {
                body.heterogeneous_rows(heights, |mut row| match rows[row.index()] {
                    InputRow::Stick(index) => {
                        let stick = &mut self.study.sticks[index];
                        let id = stick.id;
                        let selected = self.selected_input == Some(InputSelection::Stick(id));
                        let name_hint = format!("Nome stick: {}", stick.name);
                        row.col(|ui| {
                            if index > 0 {
                                ui.add_space(12.0);
                            }
                            let fill = egui::Rect::from_min_max(ui.cursor().min, ui.max_rect().max);
                            ui.painter()
                                .rect_filled(fill, 4.0, design::tokens(ui).surface);
                            design::container(ui, Surface::Main)
                                .inner_margin(10)
                                .stroke(egui::Stroke::new(
                                    if selected { 1.5 } else { 0.0 },
                                    design::tokens(ui).accent,
                                ))
                                .corner_radius(4)
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        let closed = self.collapsed.contains(&id);
                                        if ui
                                            .add_sized(
                                                [26.0, 30.0],
                                                design::button(
                                                    ui,
                                                    if closed { "+" } else { "−" },
                                                    ButtonVariant::Quiet,
                                                ),
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
                                        let label = ui.add(
                                            egui::Label::new(
                                                RichText::new(format!("{:02}", index + 1))
                                                    .strong()
                                                    .color(design::tokens(ui).accent),
                                            )
                                            .selectable(false),
                                        );
                                        changed |= ui
                                            .add_sized(
                                                [ui.available_width(), 30.0],
                                                design::field(&mut stick.name)
                                                    .id(Id::new(("stick_name", id))),
                                            )
                                            .labelled_by(label.id)
                                            .on_hover_text(name_hint)
                                            .changed();
                                    });
                                    ui.add_space(4.0);
                                    ui.columns(4, |columns| {
                                        for (column, (key, label, description, value)) in
                                            columns.iter_mut().zip([
                                                (
                                                    "length",
                                                    "Hs (mm)",
                                                    "Lunghezza stick",
                                                    &mut stick.length,
                                                ),
                                                (
                                                    "width",
                                                    "As (mm)",
                                                    "Larghezza stick",
                                                    &mut stick.width,
                                                ),
                                                (
                                                    "thickness",
                                                    "Ss (mm)",
                                                    "Spessore stick",
                                                    &mut stick.thickness,
                                                ),
                                                (
                                                    "fin",
                                                    "Bs (mm)",
                                                    "Lunghezza pinna",
                                                    &mut stick.fin,
                                                ),
                                            ])
                                        {
                                            let label = column
                                                .label(
                                                    RichText::new(label)
                                                        .size(11.0)
                                                        .color(design::tokens(column).muted),
                                                )
                                                .on_hover_text(description);
                                            changed |= column
                                                .add_sized(
                                                    [column.available_width(), 30.0],
                                                    design::field(value)
                                                        .id(Id::new((key, id)))
                                                        .horizontal_align(egui::Align::RIGHT),
                                                )
                                                .labelled_by(label.id)
                                                .on_hover_text(description)
                                                .changed();
                                        }
                                    });
                                    stick_diagram(ui);
                                    ui.horizontal(|ui| {
                                        ui.strong("Formati");
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new("Qtà")
                                                        .size(11.0)
                                                        .color(design::tokens(ui).muted),
                                                );
                                            },
                                        );
                                    });
                                });
                        });
                        let response = row.response();
                        let focused = ctx.memory(|m| m.focused());
                        if response.clicked()
                            || ["stick_name", "length", "width", "thickness", "fin"]
                                .iter()
                                .any(|key| focused == Some(Id::new((*key, id))))
                        {
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
                        let selected = self.selected_input == Some(InputSelection::Format(id));
                        let name_hint = format!("Nome formato: {}", format.name);
                        row.col(|ui| {
                            design::container(ui, Surface::Main)
                                .stroke(egui::Stroke::new(
                                    if selected { 1.5 } else { 0.0 },
                                    design::tokens(ui).accent,
                                ))
                                .inner_margin(egui::Margin::symmetric(12, 3))
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        ui.add_space(14.0);
                                        let name_width = (ui.available_width() - 80.0).max(80.0);
                                        changed |= ui
                                            .add_sized(
                                                [name_width, 30.0],
                                                design::field(&mut format.name)
                                                    .id(Id::new(("format_name", id))),
                                            )
                                            .on_hover_text(name_hint)
                                            .changed();
                                        changed |= ui
                                            .add_sized(
                                                [64.0, 30.0],
                                                design::field(&mut format.quantity)
                                                    .id(Id::new(("quantity", id)))
                                                    .horizontal_align(egui::Align::RIGHT),
                                            )
                                            .on_hover_text("Quantità di stick per cassetto")
                                            .changed();
                                    });
                                });
                        });
                        let response = row.response();
                        let focused = ctx.memory(|m| m.focused());
                        if response.clicked()
                            || ["format_name", "quantity"]
                                .iter()
                                .any(|key| focused == Some(Id::new((*key, id))))
                        {
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
        ui.spacing_mut().item_spacing.y = row_spacing;
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(
                    design::button(ui, "Agg. Stick", ButtonVariant::Secondary)
                        .min_size(egui::vec2(0.0, 32.0)),
                )
                .clicked()
            {
                self.study.add_stick();
                self.selected_stick = self.study.sticks.last().map(|s| s.id);
                self.selected_input = self.selected_stick.map(InputSelection::Stick);
                changed = true;
            }
            if ui
                .add_enabled(
                    !self.study.sticks.is_empty(),
                    design::button(ui, "Agg. Formato", ButtonVariant::Secondary)
                        .min_size(egui::vec2(0.0, 32.0)),
                )
                .on_hover_text("Aggiunge un formato allo stick selezionato, o al primo stick")
                .clicked()
            {
                self.study.add_format();
                if let Some(parent) = self
                    .selected_stick
                    .filter(|id| self.study.sticks.iter().any(|s| s.id == *id))
                {
                    self.study
                        .formats
                        .last_mut()
                        .expect("just inserted")
                        .stick_id = Some(parent);
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
                .add_enabled(
                    self.selected_input.is_some(),
                    design::button(ui, "Rimuovi", ButtonVariant::Quiet)
                        .min_size(egui::vec2(0.0, 32.0)),
                )
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
            self.study.formats.retain(|f| f.id != id);
            self.selected_input = None;
            changed = true;
        }
        ui.label(
            RichText::new("Punto o virgola per i decimali · clic destro per rimuovere")
                .size(10.0)
                .color(design::tokens(ui).muted),
        );
        changed
    }
}

/// Symbolic MT channels, retaining Np and the offset D from the original scheme.
/// This is a legend, not a geometrical prediction or a scaled result.
pub(super) fn mt_diagram(ui: &mut egui::Ui, channels: &str) {
    let t = design::tokens(ui);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 88.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let count = channels.trim().parse::<usize>().ok().filter(|n| *n >= 2);
    let shown = count.unwrap_or(12).min(24);
    let left = rect.left() + 8.0;
    let span = rect.width() - 70.0;
    for i in 0..shown {
        let x = left + span * i as f32 / (shown - 1) as f32;
        let top = rect.top() + 12.0;
        let body = egui::Rect::from_min_size(
            egui::pos2(x, top),
            egui::vec2((span / shown as f32 * 0.45).min(18.0), 43.0),
        );
        painter.rect(
            body,
            1.0,
            t.surface,
            egui::Stroke::new(1.0, t.drawing),
            egui::StrokeKind::Inside,
        );
        painter.hline(body.x_range(), top + 5.0, egui::Stroke::new(1.0, t.drawing));
        painter.hline(
            body.x_range(),
            body.bottom() - 5.0,
            egui::Stroke::new(1.0, t.drawing),
        );
        let centre = body.center().x;
        painter.line_segment(
            [
                egui::pos2(centre, body.bottom() + 4.0),
                egui::pos2(centre + 15.0, body.bottom() + 16.0),
            ],
            egui::Stroke::new(1.0, t.accent),
        );
    }
    painter.text(
        egui::pos2(rect.right() - 36.0, rect.top() + 36.0),
        egui::Align2::CENTER_CENTER,
        "Np",
        egui::FontId::proportional(13.0),
        t.accent,
    );
    painter.text(
        egui::pos2(rect.left() + 8.0, rect.bottom() - 4.0),
        egui::Align2::LEFT_BOTTOM,
        if count.is_some_and(|n| n <= 24) {
            "D · offset massimo (mm)  —  schema non in scala"
        } else {
            "Np · canali MT  /  D · offset (mm)  —  schema simbolico"
        },
        egui::FontId::proportional(10.0),
        t.muted,
    );
}

/// A dimensional legend with a plan view and a separate thickness section.
/// Labels refer to editable physical dimensions; no concept measurements are used.
fn stick_diagram(ui: &mut egui::Ui) {
    let t = design::tokens(ui);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 58.0), egui::Sense::hover());
    let p = ui.painter_at(rect);
    let body = egui::Rect::from_min_max(
        rect.min + egui::vec2(42.0, 10.0),
        egui::pos2(rect.right() - 80.0, rect.top() + 29.0),
    );
    p.rect(
        body,
        2.0,
        t.inset,
        egui::Stroke::new(1.2, t.drawing),
        egui::StrokeKind::Inside,
    );
    for x in [body.left() + 10.0, body.right() - 10.0] {
        p.vline(x, body.y_range(), egui::Stroke::new(1.0, t.drawing));
    }
    dimension(
        &p,
        egui::pos2(body.left(), body.bottom() + 9.0),
        egui::pos2(body.right(), body.bottom() + 9.0),
        "Hs",
        t.accent,
    );
    dimension(
        &p,
        egui::pos2(body.left() - 20.0, body.top()),
        egui::pos2(body.left() - 20.0, body.bottom()),
        "As",
        t.accent,
    );
    dimension(
        &p,
        egui::pos2(body.right() - 10.0, body.top() - 5.0),
        egui::pos2(body.right(), body.top() - 5.0),
        "Bs",
        t.accent,
    );
    let section = egui::Rect::from_center_size(
        egui::pos2(rect.right() - 35.0, body.center().y),
        egui::vec2(28.0, 8.0),
    );
    p.add(egui::epaint::EllipseShape::stroke(
        section.center(),
        section.size() / 2.0,
        egui::Stroke::new(1.2, t.drawing),
    ));
    p.text(
        egui::pos2(section.center().x, section.bottom() + 7.0),
        egui::Align2::CENTER_TOP,
        "Ss",
        egui::FontId::proportional(11.0),
        t.accent,
    );
}

fn dimension(p: &egui::Painter, a: egui::Pos2, b: egui::Pos2, label: &str, color: Color32) {
    let normal = (b - a).normalized().rot90() * 3.0;
    let stroke = egui::Stroke::new(1.0, color);
    p.line_segment([a, b], stroke);
    p.line_segment([a - normal, a + normal], stroke);
    p.line_segment([b - normal, b + normal], stroke);
    let vertical = (b.y - a.y).abs() > (b.x - a.x).abs();
    p.text(
        (a + b.to_vec2()) / 2.0
            + if label == "Bs" {
                egui::vec2(13.0, 0.0)
            } else if vertical {
                egui::vec2(-5.0, 0.0)
            } else {
                egui::vec2(0.0, 3.0)
            },
        if label == "Bs" {
            egui::Align2::LEFT_TOP
        } else if vertical {
            egui::Align2::RIGHT_CENTER
        } else {
            egui::Align2::CENTER_TOP
        },
        label,
        egui::FontId::proportional(11.0),
        color,
    );
}
