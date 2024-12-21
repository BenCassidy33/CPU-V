use egui::RichText;
use egui_extras::{Column, TableBuilder};
use std::{collections::BTreeMap, process::exit};

use super::app::UiApp;

// TODO: Split tables into different sizes, kinda like: https://en.wikibooks.org/wiki/X86_Assembly/X86_Architecture
pub fn render(app: &mut UiApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    let mut register_data = serde_json::from_value::<BTreeMap<String, serde_json::Value>>(
        serde_json::to_value(&app.previous_data.registers).unwrap(),
    )
    .unwrap();

    let signed_registers = register_data.split_off(&String::from("sah"));

    ui.vertical(|ui| {
        ui.set_height(ui.available_height() * 0.5);
        ui.set_width(ui.available_width() * 0.25);
        let avail_height = ui.available_height();

        ui.heading(RichText::new("Unsigned Registers").heading().strong());
        ui.add_space(20.0);

        ui.vertical(|ui| {
            render_unsigned_registers(ui, register_data, avail_height);
        });

        ui.add_space(50.0);
        ui.heading(RichText::new("Signed Registers").heading().strong());
        ui.add_space(20.0);

        // signed registers
        ui.vertical(|ui| {
            render_signed_registers(ui, signed_registers, avail_height);
        });
    });
}

pub fn render_unsigned_registers(
    ui: &mut egui::Ui,
    register_data: BTreeMap<String, serde_json::Value>,
    avail_height: f32,
) {
    let column_width = ui.available_width() / 2.0;
    ui.set_height(avail_height * 0.5);
    ui.push_id("unsigned", |ui| {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .vscroll(true)
            .column(Column::exact(column_width).resizable(true))
            .column(Column::remainder())
            .header(50.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Register");
                });

                header.col(|ui| {
                    ui.heading("Value");
                });
            })
            .body(|mut body| {
                for (key, value) in register_data.iter() {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", key));
                        });

                        row.col(|ui| {
                            ui.label(format!("{:?}", value));
                        });
                    });
                }
            });
    });
}

pub fn render_signed_registers(
    ui: &mut egui::Ui,
    register_data: BTreeMap<String, serde_json::Value>,
    avail_height: f32,
) {
    ui.set_height(avail_height * 0.5);
    let column_width = ui.available_width() / 2.0;
    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .vscroll(true)
        .column(Column::exact(column_width).resizable(true))
        .column(Column::remainder())
        .header(50.0, |mut header| {
            header.col(|ui| {
                ui.heading("Register");
            });

            header.col(|ui| {
                ui.heading("Value");
            });
        })
        .body(|mut body| {
            for (key, value) in register_data.iter() {
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("{}", key));
                    });

                    row.col(|ui| {
                        ui.label(format!("{:?}", value));
                    });
                });
            }
        });
}
