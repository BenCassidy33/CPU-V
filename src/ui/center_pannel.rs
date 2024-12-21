use egui::CentralPanel;

use super::{app::UiApp, log, visual};

pub fn render(app: &mut UiApp, ctx: &egui::Context) {
    let pannel = CentralPanel::default().show(ctx, |ui| {
        visual::render(app, ctx, ui);
        log::render(app, ctx, ui);
    });
}
