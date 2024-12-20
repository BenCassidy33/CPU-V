use egui::{Button, Ui};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use super::app::UiApp;
use crate::core::engine::ClientCommands;

pub fn render(app: &mut UiApp, ctx: &egui::Context) {
    let window_size = ctx.screen_rect().max;
    let pannel = egui::SidePanel::left("Text Editor")
        .min_width(window_size.x * 0.35)
        .show(ctx, |ui| {
            let available_height = ui.available_height();

            // text editor controls
            ui.vertical(|ui| {
                ui.set_height(available_height * 0.05);
                ui.label("File Controls");
                ui.horizontal(|ui| {
                    if ui.add(Button::new("Start")).clicked() {
                        app.command_sender.send(vec![ClientCommands::Start]);
                    };

                    if ui.add(Button::new("Stop")).clicked() {
                        app.command_sender.send(vec![ClientCommands::Stop]);
                    };

                    if ui.add(Button::new("Pause")).clicked() {
                        app.command_sender.send(vec![ClientCommands::Pause]);
                    };
                })
            });

            ui.add_space(20.0);

            // text editor
            ui.vertical(|ui| {
                //ui.set_height(available_height * 0.8);
                ui.horizontal(|ui| {
                    app.show_file_picker(ctx, ui);
                });

                app.show_code_editor(ui, ctx);
            });
        });
}
