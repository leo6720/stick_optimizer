//! Modal presentation of the worker's counters; never predicts remaining time.
use super::*;

fn count(value: u64) -> String {
    let digits = value.to_string();
    let mut result = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            result.push('.');
        }
        result.push(digit);
    }
    result
}

impl PrototypeApp {
    pub(super) fn cancel_search(&self) {
        if let Some(job) = &self.job {
            job.control.cancelled.store(true, Ordering::Relaxed);
        }
    }

    pub(super) fn search_progress(&self, ctx: &egui::Context) {
        let Some(job) = &self.job else { return };
        let cancelling = job.control.cancelled.load(Ordering::Relaxed);
        let finalizing = job.control.finalizing.load(Ordering::Acquire);
        let total = job.control.estimated.load(Ordering::Relaxed);
        let examined = job.control.examined.load(Ordering::Relaxed);
        // The last combination can complete between these loads and the phase
        // change. Show finalization then too, never premature success.
        let searching = !cancelling && !finalizing && total > 0 && examined < total;
        let width = (ctx.content_rect().width() - 64.0).clamp(180.0, 420.0);
        let mut cancel = false;
        egui::Modal::new(Id::new("search_progress"))
            .backdrop_color(Color32::from_black_alpha(45))
            .frame(egui::Frame::popup(&ctx.global_style()).inner_margin(20))
            .show(ctx, |ui| {
                ui.set_width(width);
                ui.horizontal(|ui| {
                    ui.heading(
                        RichText::new("Calcolo in corso").color(crate::ui::tokens(ui).muted),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        cancel |= ui
                            .add_enabled(!cancelling, egui::Button::new("×"))
                            .on_hover_text("Annulla calcolo")
                            .clicked();
                    });
                });
                ui.add_space(18.0);
                if searching {
                    let fraction = examined as f64 / total as f64;
                    // Truncate: 99.9% is not a completed enumeration.
                    ui.label(
                        RichText::new(format!("{}% completato", (fraction * 100.0) as u32))
                            .size(26.0)
                            .color(crate::ui::tokens(ui).muted),
                    );
                    ui.add(
                        egui::ProgressBar::new(fraction as f32)
                            .fill(crate::ui::tokens(ui).accent)
                            .desired_width(width)
                            .desired_height(8.0),
                    );
                    ui.label(
                        RichText::new(format!(
                            "{} / {} combinazioni esaminate",
                            count(examined),
                            count(total)
                        ))
                        .small()
                        .color(crate::ui::tokens(ui).muted),
                    );
                } else {
                    ui.label(
                        RichText::new(if cancelling {
                            "Annullamento…"
                        } else if finalizing || total > 0 {
                            "Preparazione risultati…"
                        } else {
                            "Generazione candidati…"
                        })
                        .size(20.0)
                        .color(crate::ui::tokens(ui).muted),
                    );
                    ui.add_space(4.0);
                    ui.spinner();
                }
                ui.add_space(18.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    cancel |= ui
                        .add_enabled(!cancelling, egui::Button::new("Annulla"))
                        .clicked();
                });
            });
        // No should_close(): Escape/backdrop never hide a running job.
        if cancel {
            self.cancel_search();
        }
    }
}

#[cfg(feature = "ui-preview")]
impl PrototypeApp {
    // Frozen display fixtures, not a simulated production calculation.
    pub(super) fn preview_progress(&mut self, mode: &str) {
        let control = Arc::new(SearchControl::default());
        if mode != "progress-preparing" {
            control.estimated.store(10_000, Ordering::Relaxed);
            control.examined.store(4_200, Ordering::Relaxed);
        }
        control
            .finalizing
            .store(mode == "progress-finalizing", Ordering::Relaxed);
        control
            .cancelled
            .store(mode == "progress-cancelling", Ordering::Relaxed);
        let worker_control = Arc::clone(&control);
        let cancellation_fixture = mode == "progress-cancelling";
        let handle = std::thread::spawn(move || {
            while !worker_control.cancelled.load(Ordering::Relaxed) {
                std::thread::park_timeout(std::time::Duration::from_millis(20));
            }
            // Keep the cancellation fixture visible for its viewport capture.
            if cancellation_fixture {
                std::thread::park_timeout(std::time::Duration::from_secs(30));
            }
            Err(SearchError::Cancelled)
        });
        self.job = Some(SearchJob {
            revision: self.revision,
            control,
            handle: Some(handle),
        });
    }
}
