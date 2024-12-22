use egui::{Button, Color32, ComboBox, RichText, ScrollArea};
use strum::IntoEnumIterator;

use super::app::{ParsingResultViewOptions, UiApp};
use crate::core::engine::{ClientCommandType, ClientCommands};

pub fn render(app: &mut UiApp, ctx: &egui::Context) {
    let window_size = ctx.screen_rect().max;
    let pannel = egui::SidePanel::left("Text Editor")
        .min_width(window_size.x * 0.2)
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

            ComboBox::from_label("Select Parsing Result View")
                .selected_text(format!("{:?}", app.ui_opts.parsing_results))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.ui_opts.parsing_results,
                        ParsingResultViewOptions::Raw,
                        "Raw",
                    );
                    ui.selectable_value(
                        &mut app.ui_opts.parsing_results,
                        ParsingResultViewOptions::Tabled,
                        "Tabled",
                    );
                    ui.selectable_value(
                        &mut app.ui_opts.parsing_results,
                        ParsingResultViewOptions::None,
                        "None",
                    );
                });

            ui.add_space(5.0);

            ui.vertical(|ui| {
                ui.set_height(ui.available_height() * 0.9);
                ui.set_width(ui.available_width());

                render_parsing_results(app, ui);
            })
        });
}

pub fn render_parsing_results(app: &mut UiApp, ui: &mut egui::Ui) {
    match app.ui_opts.parsing_results {
        ParsingResultViewOptions::Raw => ScrollArea::vertical().show(ui, |ui| {
            let data = app.previous_data.program.clone();

            let mut viewable = match data {
                Some(mut data) => &mut format!("{:#?}", &mut data).to_string(),
                None => &mut "".to_string(),
            };

            let output = egui::TextEdit::multiline(viewable)
                .desired_rows(150)
                .desired_width(ui.available_width())
                .background_color(Color32::BLACK)
                .interactive(false)
                .clip_text(true)
                .show(ui);
        }),
        ParsingResultViewOptions::Tabled => {
            let extern_functions = &app.previous_data.program;
            println!("{:?}", extern_functions);
            todo!()
        }
        ParsingResultViewOptions::None => todo!(),
    };
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
