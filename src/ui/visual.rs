use super::app::UiApp;

pub fn render(app: &mut UiApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.set_height(ui.available_height() * 0.8);
    });
}
