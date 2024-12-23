use egui::RichText;
use egui_extras::{Column, TableBuilder};
use std::collections::BTreeMap;

use super::app::UiApp;

pub fn render(app: &mut UiApp, ctx: &egui::Context) {
    if ctx.input(|i| i.key_pressed(egui::Key::P)) {
        app.sidebar_shown = true;
    }

    if !app.sidebar_shown {
        return;
    }

    let window_size = ctx.screen_rect().max;
    let _side_pannel = egui::SidePanel::right("Engine Data")
        .min_width(window_size.x * 0.15)
        .resizable(true)
        .show(ctx, |ui| {
            ui.available_height();

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

            ui.add_space(50.0);

            ui.vertical(|ui| {
                ui.heading(RichText::new("Engine Data").size(36.0).strong());

                ui.add_space(20.0);
                show_engine_data_table(
                    ui,
                    serde_json::from_value::<BTreeMap<String, serde_json::Value>>(
                        serde_json::to_value(&app.previous_data).unwrap(),
                    )
                    .unwrap(),
                );
            });
        });
}

pub fn show_engine_data_table(ui: &mut egui::Ui, data: BTreeMap<String, serde_json::Value>) {
    let ignored_entries: Vec<&str> = Vec::from([
        "program",
        "IR Repserentation",
        "Parsing Result",
        "registers",
    ]);

    let column_width = ui.available_width() / 2.0;

    TableBuilder::new(ui)
        .striped(true)
        .column(Column::exact(column_width).resizable(true))
        .column(Column::remainder())
        .header(40.0, |mut header| {
            header.col(|ui| {
                ui.heading("Key");
            });

            header.col(|ui| {
                ui.heading("Value");
            });
        })
        .body(|mut body| {
            for (key, val) in data.iter() {
                if ignored_entries.contains(&key.as_str()) {
                    continue;
                }

                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label(key);
                    });

                    row.col(|ui| {
                        ui.label(match val {
                            serde_json::Value::Number(n) => format!("{:?}", n.as_u64().unwrap()),
                            serde_json::Value::Null => "Null".to_string(),
                            serde_json::Value::Bool(n) => format!("{:?}", n.to_string()),
                            serde_json::Value::String(n) => format!("{:?}", n),
                            serde_json::Value::Array(n) => format!("{:?}", n),
                            serde_json::Value::Object(n) => format!("{:?}", n),
                        });
                    });
                })
            }
        });
}
