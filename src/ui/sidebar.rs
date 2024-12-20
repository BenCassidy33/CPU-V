use egui::RichText;
use egui_extras::{Column, TableBuilder};
use std::collections::BTreeMap;
use structmap::ToMap;

use super::app::UiApp;
use crate::core::engine::EngineData;

//use eframe::egui::Ui;

pub fn render(app: &mut UiApp, ctx: &egui::Context) {
    if ctx.input(|i| i.key_pressed(egui::Key::P)) {
        app.sidebar_shown = true;
    }

    if !app.sidebar_shown {
        return;
    }

    if let Ok(data) = app.data_recv.try_recv() {
        app.previous_data = data;
    }

    let window_size = ctx.screen_rect().max;
    let side_pannel = egui::SidePanel::left("Engine Data")
        .min_width(window_size.x * 0.15)
        .resizable(true)
        .show(ctx, |ui| {
            let available_height = ui.available_height();

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add(egui::Button::new(RichText::new("X").size(20.0).strong()))
                        .clicked()
                    {
                        app.sidebar_shown = false;
                    };

                    ui.label("(press P to show again)")
                })
            });

            ui.add_space(10.0);

            ui.vertical(|ui| {
                ui.set_height(available_height * 0.4);
                ui.heading(RichText::new("Engine Controls").size(36.0).strong())
            });

            ui.add_space(10.0);

            ui.vertical(|ui| {
                ui.set_height(available_height * 0.5);
                ui.heading(RichText::new("Engine Data").size(36.0).strong());

                ui.add_space(available_height * 0.02);

                show_engine_data_table(ui, EngineData::to_stringmap(app.previous_data.clone()));
            });
        });
}

pub fn show_engine_controls(app: &mut UiApp, ctx: &egui::Context) {}

pub fn show_engine_data_table(ui: &mut egui::Ui, data: BTreeMap<String, String>) {
    let column_width = ui.available_width() / 2.0;

    TableBuilder::new(ui)
        .striped(true)
        .column(Column::exact(column_width).resizable(true))
        .column(Column::remainder())
        .header(80.0, |mut header| {
            header.col(|ui| {
                ui.heading("Key");
            });

            header.col(|ui| {
                ui.heading("Value");
            });
        })
        .body(|mut body| {
            for (key, val) in data.iter() {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label(key);
                    });

                    row.col(|ui| {
                        ui.label(val);
                    });
                })
            }
        });
}
