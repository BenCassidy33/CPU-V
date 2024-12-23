use egui::{RichText, TopBottomPanel};

use crate::core::engine::StdLogLevel;

use super::app::UiApp;

pub fn render(app: &mut UiApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    if let Ok(log_message) = app.stdlog_reciever.try_recv() {
        if log_message.log_level == StdLogLevel::UserPrint {
            app.stdout.push(log_message.message);
        } else {
            app.system_logs.push(log_message.message);
        }
    }

    TopBottomPanel::bottom("Logs")
        .resizable(true)
        .min_height(ui.available_height() * 0.2)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.label(RichText::new("Logs & Stdout").heading().strong());
                ui.add_space(10.0);
            });

            ui.horizontal(|ui| {
                ui.label("System Logs");
                ui.add_space(ui.available_width() * 0.481);
                ui.label("Stdout");
            });

            // System Logs
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width() * 0.5);
                    egui::TextEdit::multiline(&mut app.system_logs.join("\n"))
                        .desired_rows(10)
                        .desired_width(ui.available_width())
                        .clip_text(true)
                        .interactive(false)
                        .show(ui);
                });

                // Virtual Stdout
                ui.vertical(|ui| {
                    egui::TextEdit::multiline(&mut app.stdout.join("\n"))
                        .desired_rows(10)
                        .desired_width(ui.available_width())
                        .clip_text(true)
                        .interactive(false)
                        .show(ui);
                })
            });
        });
}
