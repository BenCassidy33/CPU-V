use egui::{Button, Color32, RichText, ScrollArea};
use strum::IntoEnumIterator;

use super::app::UiApp;
use crate::core::engine::{ClientCommandType, ClientCommands};

pub fn render(app: &mut UiApp, ctx: &egui::Context) {
    let window_size = ctx.screen_rect().max;
    let pannel = egui::SidePanel::left("Text Editor")
        .min_width(window_size.x * 0.35)
        .show(ctx, |ui| {
            let available_height = ui.available_height();

            ui.label(RichText::new("File/Engine Controls").strong().size(24.0));
            // text editor controls
            ui.vertical(|ui| {
                ui.set_height(available_height * 0.05);
                ui.horizontal_wrapped(|ui| {
                    render_controls(app, ctx, ui);
                })
            });

            ui.add_space(20.0);

            ui.label(RichText::new("Text Editor").strong().size(24.0));
            ui.add_space(5.0);
            // text editor
            ui.vertical(|ui| {
                //ui.set_height(available_height * 0.8);
                ui.horizontal(|ui| {
                    app.show_file_picker(ctx, ui);
                });

                app.show_code_editor(ui, ctx);
            });

            ui.add_space(20.0);
            ui.label(RichText::new("Parsing Results").strong().size(24.0));
            ui.add_space(5.0);

            ui.vertical(|ui| {
                ui.set_height(ui.available_height() * 0.25);
                ui.set_width(ui.available_width());
                ScrollArea::vertical().show(ui, |ui| {
                    if app.previous_data.program.split("").last() == Some(")") {
                        app.previous_data.program.pop();
                    }
                    let mut data = app.previous_data.program.replace("Some(", "");

                    let output = egui::TextEdit::multiline(&mut data)
                        .desired_width(ui.available_width())
                        .background_color(Color32::BLACK)
                        .interactive(false)
                        .clip_text(true)
                        .show(ui);
                })
            })
        });
}

pub fn render_controls(app: &mut UiApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    for command_type in ClientCommandType::iter() {
        if ui
            .add(Button::new(
                RichText::new(format!("{:?}", command_type)).size(16.0),
            ))
            .clicked()
        {
            app.command_sender.send(ClientCommands {
                payload: match command_type {
                    ClientCommandType::Start
                    | ClientCommandType::ParseFile
                    | ClientCommandType::ParseWithoutUpdate => Some(app.code.clone()),
                    _ => None,
                },

                command_type,
            });
        }
    }
}
