//! Static front views; physical positions and quotations are owned by core.
use super::*;
use stick_optimizer_core::{HeadGeometry, head_diagram_dimensions};

// Preserve the core value's round-trip precision in mm labels.
fn measurement(value: f64) -> String {
    value.to_string().replace('.', ",")
}

pub(super) struct HeadPopup {
    pub rank: usize,
    pub head_id: usize,
    pub format_id: u64,
    zoom: f32,
}

impl HeadPopup {
    pub fn new(rank: usize, solution: &Solution, head_id: usize) -> Option<Self> {
        let head = solution.head_types.iter().find(|h| h.id == head_id)?;
        Some(Self {
            rank,
            head_id,
            format_id: *head.geometry.format_ids.first()?,
            zoom: 1.0,
        })
    }
}

impl PrototypeApp {
    pub(super) fn head_window(&mut self, ctx: &egui::Context) {
        #[cfg(feature = "ui-preview")]
        if self.head_popup.is_none()
            && self.job.is_none()
            && let Ok(mode) = std::env::var("STICK_OPTIMIZER_PREVIEW")
            && mode.starts_with("head-popup")
            && let Some(result) = self.results.iter().find(|r| {
                mode != "head-popup-g3"
                    || r.solution
                        .head_types
                        .iter()
                        .any(|h| h.geometry.grouping == 3)
            })
            && let Some(head) = if mode == "head-popup-g3" {
                result
                    .solution
                    .head_types
                    .iter()
                    .find(|h| h.geometry.grouping == 3)
            } else {
                result
                    .solution
                    .head_types
                    .get(if mode == "head-popup-g4" { 1 } else { 0 })
            }
        {
            self.head_popup = HeadPopup::new(result.id, &result.solution, head.id);
            if let Some(popup) = self.head_popup.as_mut() {
                if mode == "head-popup-zoom" {
                    popup.zoom = 1.5;
                }
                if mode == "head-popup-alternate" {
                    popup.format_id = *head.geometry.format_ids.last().unwrap();
                }
            }
        }
        let Some(mut popup) = self.head_popup.take() else {
            return;
        };
        let Some(result) = self.results.iter().find(|r| r.id == popup.rank) else {
            return;
        };
        let solution = Arc::clone(&result.solution);
        let Some(head) = solution.head_types.iter().find(|h| h.id == popup.head_id) else {
            return;
        };
        let candidates: Vec<_> = solution
            .candidates
            .iter()
            .filter(|c| c.head_id == Some(head.id))
            .collect();
        let Ok(study) = self.study.to_study() else {
            return;
        };
        let available = ctx.content_rect();
        let width = (available.width() - 40.0).clamp(300.0, 1000.0);
        let height = (available.height() - 60.0).clamp(280.0, 720.0);
        let mut open = true;
        let mut close = false;
        egui::Window::new(format!("Testa {} · ID {} · Soluzione {:02}", head.id + 1, head.id, popup.rank))
            .id(Id::new("head_detail_window"))
            .frame(crate::ui::dialog_frame(ctx))
            .open(&mut open).collapsible(false)
            .default_width(width).default_height(height)
            .min_size(egui::vec2(280.0, 240.0))
            .default_pos(available.center() - egui::vec2(width, height) / 2.0)
            .show(ctx, |ui| {
                crate::ui::dialog_intro(ui, "Geometria effettiva della testa · viste frontali prima e dopo il raggruppamento.");
                egui::ScrollArea::vertical().id_salt("head_content")
                    .max_height((ui.available_height() - 60.0).clamp(60.0, crate::ui::dialog_body_height(ctx, 200.0))).show(ui, |ui| {
                    crate::ui::dialog_section(ui, |ui| {
                    ui.label(format!("{} prelevatori · 1 stick per prelevatore · Raggruppamento {}", head.geometry.positions.len(), head.geometry.grouping));
                    ui.label(format!("Formati associati: {}", candidates.iter().map(|c| c.format_name.as_str()).collect::<Vec<_>>().join(", ")));
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Formato rappresentato:");
                        let selected_name = candidates.iter().find(|c| c.format_id == popup.format_id).map(|c| c.format_name.as_str()).unwrap_or("Formato");
                        egui::ComboBox::from_id_salt("head_format").selected_text(selected_name)
                            .width((ui.available_width() - 80.0).clamp(100.0, 360.0)).truncate()
                            .show_ui(ui, |ui| { for c in &candidates {
                                let label = if popup.format_id == c.format_id { format!("✓ {}", c.format_name) } else { c.format_name.clone() };
                                ui.selectable_value(&mut popup.format_id, c.format_id, label);
                            } }).response.on_hover_text(selected_name);
                    });
                    ui.add(egui::Slider::new(&mut popup.zoom, 0.5..=3.0).text("Zoom disegno"));
                    });
                    ui.add_space(8.0);
                    ui.label(RichText::new("Vista esemplificativa statica · traversa e prelevatori stilizzati, senza quote costruttive.").small().color(crate::ui::tokens(ui).muted));
                    if let Some(c) = candidates.iter().find(|c| c.format_id == popup.format_id)
                        && let Some(stick) = study.sticks.iter().find(|s| s.id == c.stick_id) {
                        ui.label(format!("{} · larghezza stick {} mm", c.format_name, measurement(stick.width)));
                        let quotes = head_diagram_dimensions(&head.geometry);
                        ui.label(format!("Passo di prelievo: {} mm", measurement(head.geometry.pickup_pitch.mm())));
                        if let Some(pitch) = quotes.group_pitch { ui.label(format!("Passo fra gruppi: {} mm", measurement(pitch))); }
                        ui.label(format!("Interassi finali nel gruppo: {} mm", quotes.final_spacings.iter().map(|&v|measurement(v)).collect::<Vec<_>>().join(" / ")));
                        for (before, title, span) in [(true, "Prima del raggruppamento", quotes.pickup_span), (false, "Dopo il raggruppamento", quotes.final_span)] {
                            ui.add_space(8.0);
                            crate::ui::dialog_section(ui, |ui| {
                            ui.strong(title);
                            ui.add_space(6.0);
                            let canvas_width = ui.available_width().max(280.0) * popup.zoom;
                            egui::ScrollArea::horizontal().id_salt(("head_drawing", before)).show(ui, |ui| {
                                draw_front(ui, &head.geometry, stick.width, before, canvas_width);
                            });
                            ui.label(format!("Distanza primo–ultimo centro: {} mm", measurement(span)));
                            });
                        }
                    }
                    #[cfg(feature = "ui-preview")]
                    if std::env::var("STICK_OPTIMIZER_PREVIEW").is_ok_and(|m| m.ends_with("-bottom")) { ui.scroll_to_cursor(Some(egui::Align::BOTTOM)); }
                });
                close = crate::ui::dialog_close(ui);
                close |= ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
            });
        if open && !close {
            self.head_popup = Some(popup);
        }
    }
}

fn draw_front(ui: &mut egui::Ui, head: &HeadGeometry, stick_width: f64, before: bool, width: f32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 136.0), egui::Sense::hover());
    let Some(last) = head.positions.last() else {
        return;
    };
    // Common screen scale/origin keeps fixed channels aligned between both views.
    let scale = (width - 56.0) / (last.pickup_x + stick_width) as f32;
    let x = |value: f64| rect.left() + 28.0 + ((value + stick_width / 2.0) as f32 * scale);
    let painter = ui.painter_at(rect);
    let structure = crate::ui::tokens(ui).drawing;
    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(rect.left() + 12.0, rect.top() + 12.0),
            egui::pos2(rect.right() - 12.0, rect.top() + 19.0),
        ),
        0.0,
        structure,
    );
    for point in &head.positions {
        let centre = x(if before {
            point.pickup_x
        } else {
            point.final_x
        });
        painter.line_segment(
            [
                egui::pos2(centre, rect.top() + 19.0),
                egui::pos2(centre, rect.top() + 49.0),
            ],
            egui::Stroke::new(2.0, structure),
        );
        painter.rect_filled(
            egui::Rect::from_center_size(
                egui::pos2(centre, rect.top() + 49.0),
                egui::vec2(6.0, 14.0),
            ),
            0.0,
            structure,
        );
        painter.rect_filled(
            egui::Rect::from_center_size(
                egui::pos2(centre, rect.top() + 79.0),
                egui::vec2(stick_width as f32 * scale, 42.0),
            ),
            1.0,
            crate::ui::tokens(ui).product,
        );
        painter.circle_filled(egui::pos2(centre, rect.top() + 105.0), 2.0, structure);
    }
    if let (Some(first), Some(last)) = (head.positions.first(), head.positions.last()) {
        let a = x(if before {
            first.pickup_x
        } else {
            first.final_x
        });
        let b = x(if before { last.pickup_x } else { last.final_x });
        painter.line_segment(
            [
                egui::pos2(a, rect.top() + 119.0),
                egui::pos2(b, rect.top() + 119.0),
            ],
            egui::Stroke::new(1.0, crate::ui::tokens(ui).muted),
        );
        for end in [a, b] {
            painter.line_segment(
                [
                    egui::pos2(end, rect.top() + 113.0),
                    egui::pos2(end, rect.top() + 125.0),
                ],
                egui::Stroke::new(1.0, crate::ui::tokens(ui).muted),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stick_optimizer_core::{HeadFormat, Pitch, Settings, final_head_geometry};

    #[test]
    fn front_views_draw_one_real_width_stick_per_channel_at_public_core_coordinates() {
        for (grouping, first) in [
            (2, vec![0.0, 27.0]),
            (3, vec![33.0, 60.0, 87.0]),
            (4, vec![33.0, 60.0, 120.0, 147.0]),
        ] {
            let formats: Vec<_> = [25.0, 26.0]
                .iter()
                .enumerate()
                .map(|(id, &stick_width)| HeadFormat {
                    format_id: id as u64,
                    stick_width,
                    pickup_pitch: Pitch(60_000_000),
                    grouping,
                    dividers: 0,
                })
                .collect();
            let head = final_head_geometry(&Settings::default(), &formats).unwrap();
            let original = head.clone();
            for theme in [
                stick_optimizer_io::Theme::Light,
                stick_optimizer_io::Theme::Dark,
            ] {
                for width in [25.0, 26.0] {
                    for before in [true, false] {
                        let ctx = egui::Context::default();
                        crate::ui::install(&ctx);
                        crate::ui::apply_theme(&ctx, theme);
                        let mut origin = egui::Pos2::ZERO;
                        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                            origin = ui.cursor().min;
                            draw_front(ui, &head, width, before, 1000.0);
                        });
                        output.textures_delta.clear();
                        let sticks: Vec<_> = output
                            .shapes
                            .iter()
                            .filter_map(|s| match &s.shape {
                                egui::epaint::Shape::Rect(r)
                                    if (r.rect.height() - 42.0).abs() < 0.001 =>
                                {
                                    Some(r.rect)
                                }
                                _ => None,
                            })
                            .collect();
                        assert_eq!(sticks.len(), 12);
                        let grippers = output.shapes.iter().filter(|s| matches!(&s.shape, egui::epaint::Shape::Rect(r) if (r.rect.height() - 14.0).abs() < 0.001 && (r.rect.width() - 6.0).abs() < 0.001)).count();
                        assert_eq!(grippers, 12);
                        for (i, rect) in sticks.iter().enumerate() {
                            let expected = if before {
                                i as f64 * 60.0
                            } else {
                                first[i % grouping as usize]
                                    + (i / grouping as usize) as f64 * f64::from(grouping) * 60.0
                            };
                            let scale = 944.0 / (660.0 + width) as f32;
                            assert!(
                                (rect.center().x
                                    - (origin.x + 28.0 + (expected + width / 2.0) as f32 * scale))
                                    .abs()
                                    < 0.001
                            );
                            assert!((rect.width() - width as f32 * scale).abs() < 0.001);
                        }
                    }
                }
            }
            assert_eq!(head, original);
        }
    }
}
