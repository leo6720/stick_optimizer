//! Shared presentation system. Domain values and widget IDs stay at call sites.
use eframe::egui::{self, Color32, RichText};
use stick_optimizer_io::Theme;

#[derive(Clone, Copy)]
pub struct Tokens {
    pub surface: Color32,
    pub inset: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub border: Color32,
    pub accent: Color32,
    pub primary: Color32,
    pub on_primary: Color32,
    pub hover: Color32,
    pub drawing: Color32,
    pub product: Color32,
    pub product_fill: Color32,
}
impl Tokens {
    pub fn for_theme(theme: Theme) -> Self {
        let rgb = Color32::from_rgb;
        match theme {
            Theme::Light => Self {
                surface: Color32::WHITE,
                inset: rgb(245, 246, 248),
                text: rgb(27, 32, 40),
                muted: rgb(79, 88, 103),
                border: rgb(116, 126, 141),
                accent: rgb(181, 31, 32),
                primary: rgb(181, 31, 32),
                on_primary: Color32::WHITE,
                hover: rgb(248, 233, 233),
                drawing: rgb(50, 57, 67),
                product: rgb(181, 31, 32),
                product_fill: rgb(248, 220, 220),
            },
            Theme::Dark => Self {
                surface: rgb(24, 29, 35),
                inset: rgb(31, 37, 45),
                text: rgb(236, 239, 244),
                muted: rgb(183, 193, 207),
                border: rgb(114, 126, 143),
                accent: rgb(255, 113, 121),
                primary: rgb(181, 31, 32),
                on_primary: Color32::WHITE,
                hover: rgb(67, 43, 50),
                drawing: rgb(222, 229, 239),
                product: rgb(255, 113, 121),
                product_fill: rgb(92, 43, 50),
            },
        }
    }
}
pub fn tokens(ui: &egui::Ui) -> Tokens {
    Tokens::for_theme(if ui.visuals().dark_mode {
        Theme::Dark
    } else {
        Theme::Light
    })
}

pub fn install(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Inter".into(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/InterVariable.ttf"
        ))),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inter".into());
    ctx.set_fonts(fonts);
    // Both maps are installed once. Theme switching never follows the OS.
    for theme in [Theme::Light, Theme::Dark] {
        let t = Tokens::for_theme(theme);
        let egui_theme = match theme {
            Theme::Light => egui::Theme::Light,
            Theme::Dark => egui::Theme::Dark,
        };
        let mut style = egui::Style {
            visuals: if theme == Theme::Dark {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            },
            ..Default::default()
        };
        style.spacing.item_spacing = egui::vec2(6.0, 4.0);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        style.spacing.window_margin = egui::Margin::same(12);
        style.spacing.menu_margin = egui::Margin::same(8);
        style.spacing.menu_width = 340.0;
        style.visuals.window_fill = t.surface;
        style.visuals.panel_fill = t.surface;
        style.visuals.extreme_bg_color = t.surface;
        style.visuals.faint_bg_color = t.inset;
        style.visuals.text_edit_bg_color = Some(t.surface);
        style.visuals.selection.bg_fill = t.primary;
        style.visuals.selection.stroke = egui::Stroke::new(1.0, t.on_primary);
        style.visuals.hyperlink_color = t.accent;
        style.visuals.error_fg_color = t.accent;
        style.visuals.warn_fg_color = t.accent;
        for widget in [
            &mut style.visuals.widgets.noninteractive,
            &mut style.visuals.widgets.inactive,
            &mut style.visuals.widgets.hovered,
            &mut style.visuals.widgets.active,
            &mut style.visuals.widgets.open,
        ] {
            widget.corner_radius = egui::CornerRadius::same(4);
            widget.fg_stroke = egui::Stroke::new(1.0, t.text);
            widget.bg_stroke = egui::Stroke::new(1.0, t.border);
            widget.bg_fill = t.surface;
            widget.weak_bg_fill = t.inset;
        }
        style.visuals.widgets.hovered.bg_fill = t.hover;
        style.visuals.widgets.hovered.weak_bg_fill = t.hover;
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, t.accent);
        style.visuals.widgets.active.bg_fill = t.hover;
        style.visuals.widgets.active.weak_bg_fill = t.hover;
        // egui also uses active.fg_stroke for strong labels outside buttons.
        // Keep it readable on ordinary surfaces; primary actions set on_primary explicitly.
        style.visuals.widgets.active.fg_stroke.color = t.text;
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.5, t.accent);
        style.visuals.widgets.open.bg_fill = t.hover;
        style.visuals.widgets.open.weak_bg_fill = t.hover;
        for (kind, size) in [
            (egui::TextStyle::Body, 13.0),
            (egui::TextStyle::Button, 13.0),
            (egui::TextStyle::Heading, 17.0),
        ] {
            style
                .text_styles
                .insert(kind, egui::FontId::proportional(size));
        }
        ctx.set_style_of(egui_theme, style);
    }
}
pub fn apply_theme(ctx: &egui::Context, theme: Theme) {
    ctx.set_theme(match theme {
        Theme::Light => egui::Theme::Light,
        Theme::Dark => egui::Theme::Dark,
    });
    ctx.request_repaint();
}

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Quiet,
}
/// egui retains hover, keyboard focus, active and disabled behavior for each variant.
pub fn button<'a>(ui: &egui::Ui, label: &'a str, variant: ButtonVariant) -> egui::Button<'a> {
    let t = tokens(ui);
    match variant {
        ButtonVariant::Primary => {
            egui::Button::new(RichText::new(label).color(t.on_primary)).fill(t.primary)
        }
        ButtonVariant::Secondary => egui::Button::new(label),
        ButtonVariant::Quiet => egui::Button::new(label).frame(false),
    }
}
pub fn field(value: &mut String) -> egui::TextEdit<'_> {
    egui::TextEdit::singleline(value).margin(egui::vec2(3.0, 2.0))
}

/// Internal dialogs share surfaces and spacing; native pickers remain native.
pub fn dialog_frame(ctx: &egui::Context) -> egui::Frame {
    egui::Frame::window(&ctx.global_style())
        .inner_margin(16)
        .corner_radius(8)
}

pub fn dialog_intro(ui: &mut egui::Ui, description: &str) {
    ui.label(
        RichText::new(description)
            .size(12.0)
            .color(tokens(ui).muted),
    );
    ui.add_space(10.0);
}

pub fn dialog_section<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    container(ui, Surface::Inset)
        .inner_margin(12)
        .corner_radius(6)
        .show(ui, add)
        .inner
}

/// A label and border communicate an error even without colour perception.
pub fn error_notice(ui: &mut egui::Ui, message: &str) {
    let t = tokens(ui);
    container(ui, Surface::Inset)
        .inner_margin(10)
        .stroke(egui::Stroke::new(1.0, t.accent))
        .show(ui, |ui| {
            ui.label(RichText::new("Attenzione").strong().color(t.accent));
            ui.label(RichText::new(message).color(t.text));
        });
}

/// Leave the title and footer outside the scrolling body, also at 150% zoom.
pub fn dialog_body_height(ctx: &egui::Context, reserved: f32) -> f32 {
    (ctx.content_rect().height() - reserved).clamp(60.0, 560.0)
}

pub fn dialog_actions(ui: &mut egui::Ui, accept: &str) -> (bool, bool) {
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        let cancel = ui
            .add_sized(
                [90.0, 32.0],
                button(ui, "Annulla", ButtonVariant::Secondary),
            )
            .clicked();
        let apply = ui
            .add_sized([90.0, 32.0], button(ui, accept, ButtonVariant::Primary))
            .clicked();
        (cancel, apply)
    })
    .inner
}
#[cfg(feature = "ui-preview")]
pub fn heading(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.heading(RichText::new(label).color(tokens(ui).text))
}
/// Section hierarchy shared by input and output, using the same theme maps.
pub fn section_heading(ui: &mut egui::Ui, title: &str, description: &str) {
    ui.label(
        RichText::new(title)
            .size(22.0)
            .strong()
            .color(tokens(ui).text),
    );
    ui.label(
        RichText::new(description)
            .size(12.0)
            .color(tokens(ui).muted),
    );
}

pub fn brand_mark(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::hover());
    for (i, height) in [14.0, 22.0, 18.0, 24.0, 16.0].into_iter().enumerate() {
        let bar = egui::Rect::from_center_size(
            egui::pos2(rect.left() + 2.0 + i as f32 * 4.5, rect.center().y),
            egui::vec2(2.5, height),
        );
        ui.painter().rect_filled(bar, 1.0, tokens(ui).accent);
    }
}
#[derive(Clone, Copy)]
pub enum Surface {
    Main,
    Inset,
}
pub fn container(ui: &egui::Ui, surface: Surface) -> egui::Frame {
    let t = tokens(ui);
    egui::Frame::new().fill(match surface {
        Surface::Main => t.surface,
        Surface::Inset => t.inset,
    })
}
pub fn table_label(ui: &mut egui::Ui, value: &str, right: bool, selected: bool, strong: bool) {
    let layout = if right {
        egui::Layout::right_to_left(egui::Align::Center)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    ui.with_layout(layout, |ui| {
        let mut text = RichText::new(value).color(if selected {
            ui.visuals().selection.stroke.color
        } else {
            tokens(ui).text
        });
        if strong {
            text = text.strong();
        }
        // Row clicks belong to the table, not to text selection inside a label.
        ui.add(egui::Label::new(text).truncate().selectable(false))
            .on_hover_text(value);
    });
}

/// Dense output tables use a quiet selected surface plus explicit row markers.
/// Scope these overrides so input selection and popup tables retain their styles.
pub fn data_surface<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let t = tokens(ui);
    ui.scope(|ui| {
        ui.visuals_mut().selection.bg_fill = t.hover;
        ui.visuals_mut().selection.stroke.color = t.text;
        container(ui, Surface::Main)
            .stroke(egui::Stroke::new(1.0, t.border))
            .corner_radius(4)
            .show(ui, add)
            .inner
    })
    .inner
}

pub fn data_heading(ui: &mut egui::Ui, label: &str, right: bool) {
    ui.painter().rect_filled(ui.max_rect(), 0, tokens(ui).inset);
    table_label(ui, label, right, false, true);
}

/// A persistent underline and weight identify the active detail without colour alone.
pub fn detail_selector(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let mut text = RichText::new(label).color(tokens(ui).text);
    if selected {
        text = text.strong();
    }
    let response = ui.add_sized(
        [0.0, 32.0],
        egui::Button::new(text).frame(false).selected(selected),
    );
    if selected {
        let rect = response.rect;
        ui.painter().line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            egui::Stroke::new(3.0, tokens(ui).accent),
        );
    }
    response
}

pub fn metric(ui: &mut egui::Ui, value: &str, label: &str) {
    ui.vertical(|ui| {
        ui.label(
            RichText::new(value)
                .size(21.0)
                .strong()
                .color(tokens(ui).text),
        );
        ui.label(RichText::new(label).size(12.0).color(tokens(ui).muted));
    });
}

/// Actual egui components, available only in the existing screenshot QA build.
#[cfg(feature = "ui-preview")]
pub fn gallery(ui: &mut egui::Ui) {
    egui::CentralPanel::default().show(ui, |ui| {
        heading(ui, "Fondazione UI · componenti condivisi");
        ui.label("Anteprima QA: varianti e stati egui, stessa libreria dell’app.");
        ui.separator();
        for variant in [
            ButtonVariant::Primary,
            ButtonVariant::Secondary,
            ButtonVariant::Quiet,
        ] {
            ui.horizontal(|ui| {
                ui.add(button(ui, "Azione", variant));
                ui.add_enabled(false, button(ui, "Disabilitata", variant));
                let response = ui.add(button(ui, "Focus tastiera", variant));
                if matches!(variant, ButtonVariant::Secondary) {
                    response.request_focus();
                }
            });
        }
        ui.separator();
        let mut value = "25,00".to_owned();
        ui.horizontal(|ui| {
            ui.label("Campo numerico (mm)");
            ui.add(field(&mut value).desired_width(100.0));
            ui.add_enabled(false, field(&mut value).desired_width(100.0));
            egui::ComboBox::from_id_salt("gallery_selector")
                .selected_text("Selettore")
                .show_ui(ui, |ui| {
                    ui.selectable_label(true, "Opzione scelta").clicked();
                });
        });
        ui.colored_label(tokens(ui).accent, "Errore: valore da correggere");
        container(ui, Surface::Inset)
            .inner_margin(12)
            .show(ui, |ui| {
                heading(ui, "Contenitore e intestazione");
                ui.allocate_ui(egui::vec2(320.0, 24.0), |ui| {
                    table_label(ui, "Formato", false, false, true);
                });
                ui.allocate_ui(egui::vec2(320.0, 24.0), |ui| {
                    table_label(ui, "Valore reale · 25 mm", false, false, false);
                });
                ui.label(RichText::new("Testo secondario").color(tokens(ui).muted));
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn luminance(c: Color32) -> f64 {
        let linear = |x: u8| {
            let v = f64::from(x) / 255.0;
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * linear(c.r()) + 0.7152 * linear(c.g()) + 0.0722 * linear(c.b())
    }
    fn contrast(a: Color32, b: Color32) -> f64 {
        let (a, b) = (luminance(a), luminance(b));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }
    #[test]
    fn rendered_strong_labels_are_readable_in_both_themes() {
        for theme in [Theme::Light, Theme::Dark] {
            let ctx = egui::Context::default();
            install(&ctx);
            apply_theme(&ctx, theme);
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                ui.strong("Intestazione");
            });
            output.textures_delta.clear();
            let text = output
                .shapes
                .iter()
                .find_map(|s| match &s.shape {
                    egui::epaint::Shape::Text(t) if t.galley.job.text == "Intestazione" => Some(t),
                    _ => None,
                })
                .unwrap();
            let color = text.override_text_color.unwrap_or_else(|| {
                let color = text.galley.job.sections[0].format.color;
                if color == Color32::PLACEHOLDER {
                    text.fallback_color
                } else {
                    color
                }
            });
            assert!(
                contrast(color, Tokens::for_theme(theme).inset) >= 4.5,
                "{theme:?}: rendered strong label is unreadable"
            );
        }
    }

    #[test]
    fn active_detail_selector_text_is_readable_on_both_themes() {
        for theme in [Theme::Light, Theme::Dark] {
            let ctx = egui::Context::default();
            install(&ctx);
            apply_theme(&ctx, theme);
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    detail_selector(ui, "Panoramica formati", true);
                });
            });
            let text = output
                .shapes
                .iter()
                .find_map(|s| match &s.shape {
                    egui::epaint::Shape::Text(t) if t.galley.job.text == "Panoramica formati" => {
                        Some(t)
                    }
                    _ => None,
                })
                .expect("active selector label must render");
            let color = text.override_text_color.unwrap_or_else(|| {
                let c = text.galley.job.sections[0].format.color;
                if c == Color32::PLACEHOLDER {
                    text.fallback_color
                } else {
                    c
                }
            });
            assert!(contrast(color, Tokens::for_theme(theme).surface) >= 4.5);
            output.textures_delta.clear();
        }
    }

    #[test]
    fn semantic_foregrounds_remain_readable_on_both_surface_maps() {
        for theme in [Theme::Light, Theme::Dark] {
            let t = Tokens::for_theme(theme);
            for background in [t.surface, t.inset, t.hover] {
                for foreground in [t.text, t.muted, t.accent] {
                    assert!(
                        contrast(foreground, background) >= 4.5,
                        "{theme:?}: text contrast"
                    );
                }
                assert!(
                    contrast(t.border, background) >= 3.0,
                    "{theme:?}: essential boundaries"
                );
                assert!(
                    contrast(t.drawing, background) >= 3.0,
                    "{theme:?}: diagram structure"
                );
            }
            assert!(contrast(t.on_primary, t.primary) >= 4.5);
            assert!(contrast(t.product, t.product_fill) >= 3.0);
        }
    }
}
