use std::collections::BTreeMap;

use egui::{Button, Color32, ComboBox, RichText, ScrollArea};
use egui_extras::{Column, TableBuilder};
use strum::IntoEnumIterator;

use super::app::{ParsingResultViewOptions, UiApp};
use crate::core::lang::{Label, Program};
use crate::core::{
    engine::{ClientCommandType, ClientCommands},
    lang::Variable,
};

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
    if app.previous_data.program.is_none() {
        ui.label("Cannot Display Data: No Parsing Result Found!");
        return;
    }

    match app.ui_opts.parsing_results {
        ParsingResultViewOptions::Raw => {
            ScrollArea::vertical().show(ui, |ui| {
                let mut viewable =
                    format!("{:#?}", app.previous_data.program.clone().unwrap()).to_string();

                let output = egui::TextEdit::multiline(&mut viewable)
                    .desired_rows(150)
                    .desired_width(ui.available_width())
                    .background_color(Color32::BLACK)
                    .interactive(false)
                    .clip_text(true)
                    .show(ui);
            });
        }
        ParsingResultViewOptions::Tabled => {
            let data = &app.previous_data.program.clone().unwrap();

            if let Some(d) = &data.extern_functions {
                render_extern_functions(d, ui);
            }

            if let Some(d) = Some(&data.static_variables) {
                render_static_variables(d, ui);
            }

            if let Some(d) = &data.labels {
                render_labels(d, ui);
            }
        }

        ParsingResultViewOptions::None => {}
    };
}

pub fn render_extern_functions(funcs: &Vec<String>, ui: &mut egui::Ui) {
    let avail_width = ui.available_width();
    ui.add_space(30.0);
    TableBuilder::new(ui)
        .id_salt("extern funcs table")
        .striped(true)
        .resizable(true)
        .vscroll(true)
        .column(Column::exact(avail_width).resizable(true))
        .header(30.0, |mut header| {
            header.col(|ui| {
                ui.heading("External Function Imports");
            });
        })
        .body(|mut body| {
            for func in funcs {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("{}", func));
                    });
                });
            }
        });
}
pub fn render_static_variables(vars: &Vec<Variable>, ui: &mut egui::Ui) {
    let avail_width = ui.available_width() / 3.0;
    ui.add_space(30.0);
    ui.push_id("variable_table", |ui| {
        TableBuilder::new(ui)
            .id_salt("variable_table")
            .striped(true)
            .resizable(true)
            .vscroll(true)
            .column(Column::exact(avail_width).resizable(true))
            .column(Column::exact(avail_width).resizable(true))
            .column(Column::remainder().resizable(true))
            .header(30.0, |mut header| {
                for head in vec!["Name", "Data Type", "Value"] {
                    header.col(|ui| {
                        ui.heading(head);
                        ui.add_space(10.0)
                    });
                }
            })
            .body(|mut body| {
                for var in vars {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", var.name));
                        });

                        row.col(|ui| {
                            ui.label(format!("{:?}", var.ty).to_uppercase());
                        });

                        row.col(|ui| {
                            ui.label(format!("{}", var.inital_value.to_string()));
                        });
                    })
                }
            });
    });
}
pub fn render_labels(labels: &Vec<Label>, ui: &mut egui::Ui) {}

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
