use crate::ui::window::App;

pub fn render_engine_data(app: &App, ctx: &egui::Context) {
    let engine_data = app.engine_data_reciever.try_recv();

    let window = egui::Window::new("Debug Info")
        //.default_width(WINDOW_WIDTH / 2.0)
        .resizable([true, true])
        .open(&mut true)
        .show(ctx, |ui| {
            if let Ok(data) = engine_data {
                ui.label(format!(
                    "
                        Tick: {},
                        Last Tick Time: {},
                        Lines Per Tick: {},
                        Estimated Ticks Per Second: {}
                    ",
                    data.tick,
                    data.last_tick_time,
                    data.lines_per_tick,
                    data.estimated_ticks_per_second,
                ))
            } else {
                ui.label("Could not read engine data.")
            }
        });
}
