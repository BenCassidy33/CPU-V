use egui::Button;

use super::app::UiApp;
use crate::core::engine::{ClientCommandType, ClientCommands};

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
                        app.command_sender.send(ClientCommands {
                            command_type: ClientCommandType::Start,
                            payload: Some(app.code.clone()),
                        });
                    };

                    if ui.add(Button::new("Stop")).clicked() {
                        app.command_sender.send(ClientCommands {
                            command_type: ClientCommandType::Stop,
                            payload: None,
                        });
                    };

                    if ui.add(Button::new("Pause")).clicked() {
                        app.command_sender.send(ClientCommands {
                            command_type: ClientCommandType::Pause,
                            payload: None,
                        });
                    };
                })
            });

            ui.add_space(20.0);

            // text editor
            ui.vertical(|ui| {
                println!("Code: {:?}", app.parser_result);
                //ui.set_height(available_height * 0.8);
                ui.horizontal(|ui| {
                    app.show_file_picker(ctx, ui);
                });

                app.show_code_editor(ui, ctx);
            });

            ui.vertical(|ui| ui.add(egui::TextEdit::multiline(&mut app.parser_result)))
        });
}
