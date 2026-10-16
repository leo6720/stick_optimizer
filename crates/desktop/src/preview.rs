//! Capture this app's rendered viewport for local visual QA only.

use eframe::egui;
use image::{
    ImageEncoder,
    codecs::png::{CompressionType, FilterType, PngEncoder},
};

fn save_preview(
    path: &str,
    pixels: &[u8],
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::io::BufWriter::new(std::fs::File::create(path)?);
    PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::NoFilter).write_image(
        pixels,
        width,
        height,
        image::ExtendedColorType::Rgba8,
    )?;
    Ok(())
}

pub fn capture(ctx: &egui::Context) {
    let Ok(path) = std::env::var("STICK_OPTIMIZER_SCREENSHOT_TO") else {
        return;
    };
    let screenshot = ctx.input(|input| {
        input.events.iter().find_map(|event| match event {
            egui::Event::Screenshot { image, .. } => Some(image.clone()),
            _ => None,
        })
    });
    if let Some(screenshot) = screenshot {
        let pixels: Vec<u8> = screenshot
            .pixels
            .iter()
            .flat_map(|pixel| pixel.to_array())
            .collect();
        if let Err(error) = save_preview(
            &path,
            &pixels,
            screenshot.width() as u32,
            screenshot.height() as u32,
        ) {
            eprintln!("Cannot save GUI preview: {error}");
            std::process::exit(1);
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    } else {
        // The public viewport API reads before buffer swap. Let layout and DPI
        // settle first; reading after swap can capture a discarded back buffer.
        if ctx.cumulative_pass_nr() >= 5 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
        }
        ctx.request_repaint();
    }
}
