#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod beta;
mod draft;
#[cfg(feature = "ui-preview")]
mod preview;
mod ui;

fn main() -> eframe::Result {
    #[cfg(feature = "ui-preview")]
    let window_size = if matches!(
        std::env::var("STICK_OPTIMIZER_PREVIEW").as_deref(),
        Ok("compact"
            | "detail-compact"
            | "parameters-compact"
            | "progress-compact"
            | "head-popup-compact"
            | "head-popup-compact-bottom")
    ) {
        [760.0, 620.0]
    } else {
        [1450.0, 900.0]
    };
    #[cfg(feature = "ui-preview")]
    let window_size = std::env::var("STICK_OPTIMIZER_PREVIEW_SIZE")
        .ok()
        .and_then(|s| {
            let (w, h) = s.split_once('x')?;
            Some([
                w.parse::<f32>().ok()?.max(720.0),
                h.parse::<f32>().ok()?.max(560.0),
            ])
        })
        .unwrap_or(window_size);
    #[cfg(not(feature = "ui-preview"))]
    let window_size = [1450.0, 900.0];
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(window_size)
            .with_min_inner_size([720.0, 560.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Stick Optimizer — Beta Rust",
        options,
        Box::new(|cc| Ok(Box::new(app::PrototypeApp::new(&cc.egui_ctx)))),
    )
}
