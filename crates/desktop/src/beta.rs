use eframe::egui::{self, Color32, RichText};
use stick_optimizer_io::Range;

#[derive(Default)]
pub struct FilterEditor {
    pub open: bool,
    pub values: [[String; 2]; 5],
    pub error: Option<String>,
}
impl FilterEditor {
    pub fn edit(&mut self, ranges: [Range; 5]) {
        self.values = ranges.map(|r| {
            [
                r.min.map(|v| v.to_string()).unwrap_or_default(),
                r.max.map(|v| v.to_string()).unwrap_or_default(),
            ]
        });
        self.error = None;
        self.open = true;
    }
    pub fn parsed(&self) -> Result<[Range; 5], String> {
        let mut result = [Range::default(); 5];
        for (i, values) in self.values.iter().enumerate() {
            let parse = |s: &str| {
                if s.trim().is_empty() {
                    Ok(None)
                } else {
                    crate::draft::parse_decimal(s)
                        .map(Some)
                        .map_err(str::to_owned)
                }
            };
            let r = Range {
                min: parse(&values[0])?,
                max: parse(&values[1])?,
            };
            r.validate().map_err(|e| e.to_string())?;
            result[i] = r;
        }
        Ok(result)
    }
    pub fn show(&mut self, ctx: &egui::Context) -> Option<[Range; 5]> {
        if !self.open {
            return None;
        }
        let mut open = true;
        let mut applied = None;
        egui::Window::new("Filtri delle soluzioni")
            .open(&mut open)
            .default_width(400.0)
            .default_pos(ctx.content_rect().center() - egui::vec2(210.0, 150.0))
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(
                    "Intervalli inclusivi; vuoto = nessun limite. Min = max per un valore esatto.",
                );
                egui::Grid::new("range_filters")
                    .spacing([16.0, 6.0])
                    .show(ui, |ui| {
                        ui.strong("Colonna");
                        ui.strong("Minimo");
                        ui.strong("Massimo");
                        ui.end_row();
                        for (label, values) in [
                            "Passo (mm)",
                            "Tipi cassetto",
                            "Tipi testa",
                            "Strati max",
                            "Punteggio",
                        ]
                        .into_iter()
                        .zip(&mut self.values)
                        {
                            ui.label(label);
                            for value in values {
                                ui.add(egui::TextEdit::singleline(value).desired_width(85.0));
                            }
                            ui.end_row();
                        }
                    });
                if let Some(error) = &self.error {
                    ui.colored_label(Color32::DARK_RED, error);
                }
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Annulla").clicked() {
                        self.open = false;
                    }
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Applica").color(Color32::WHITE))
                                .fill(Color32::from_rgb(181, 31, 32)),
                        )
                        .clicked()
                    {
                        match self.parsed() {
                            Ok(r) => applied = Some(r),
                            Err(e) => self.error = Some(e),
                        }
                    }
                });
                if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                    self.open = false;
                }
            });
        self.open &= open && applied.is_none();
        applied
    }
}

pub fn draw_geometry(ui: &mut egui::Ui, g: &stick_optimizer_core::Geometry) {
    ui.label(RichText::new("Sezione schematica · quote in mm · contorno alto B astuccio").small());
    ui.label(RichText::new("Vista semplificata: un solo strato completo per cassetto.").small());
    let width = ui.available_width().max(250.0);
    let height = ((f64::from(width - 48.0) * g.height / g.width + 64.0) as f32).clamp(140.0, 260.0);
    let (area, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let inner = area.shrink2(egui::vec2(24.0, 32.0));
    let scale = (f64::from(inner.width()) / g.width).min(f64::from(inner.height()) / g.height);
    let origin = egui::pos2(
        inner.center().x - (g.width * scale / 2.0) as f32,
        inner.center().y + (g.height * scale / 2.0) as f32,
    );
    let rect = |r: &stick_optimizer_core::GeometryRect| {
        egui::Rect::from_min_size(
            origin + egui::vec2((r.x * scale) as f32, (-((r.y + r.height) * scale)) as f32),
            egui::vec2((r.width * scale) as f32, (r.height * scale) as f32),
        )
    };
    let painter = ui.painter_at(area);
    for r in &g.pockets {
        painter.rect_stroke(
            rect(r),
            0,
            egui::Stroke::new(1.5, Color32::DARK_GRAY),
            egui::StrokeKind::Inside,
        );
    }
    for r in &g.dividers {
        painter.rect_filled(rect(r), 0, Color32::GRAY);
    }
    for r in &g.sticks {
        painter.rect_filled(rect(r), 1, Color32::from_rgb(248, 220, 220));
        painter.rect_stroke(
            rect(r),
            1,
            egui::Stroke::new(1.0, Color32::from_rgb(181, 31, 32)),
            egui::StrokeKind::Inside,
        );
    }
    painter.text(
        egui::pos2(area.center().x, area.bottom() - 8.0),
        egui::Align2::CENTER_BOTTOM,
        format!(
            "Passo {} mm  ·  B {} mm",
            crate::app::format_number(g.width),
            crate::app::format_number(g.height)
        ),
        egui::FontId::proportional(13.0),
        Color32::DARK_GRAY,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_editor_parses_italian_decimals_and_rejects_invalid_limits() {
        let mut editor = FilterEditor::default();
        editor.values[0] = ["200,5".into(), "210".into()];
        assert_eq!(
            editor.parsed().unwrap()[0],
            Range {
                min: Some(200.5),
                max: Some(210.0)
            }
        );
        for pair in [["300", "200"], ["NaN", ""], ["-1", ""]] {
            editor.values[0] = pair.map(str::to_owned);
            assert!(editor.parsed().is_err());
        }
    }
}
