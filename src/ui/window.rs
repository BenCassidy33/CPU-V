use eframe::egui;
use std::sync::mpsc;

use crate::core::engine::{ClientCommands, EngineData, StdLogMessage};
use crate::ui::app::UiApp;

pub const WINDOW_WIDTH: f32 = 2560.0;
pub const WINDOW_HEIGHT: f32 = 1440.0;

pub fn init(
    client_command_sender: mpsc::Sender<ClientCommands>,
    engine_data_recv: mpsc::Receiver<EngineData>,
    stdlog_reciever: mpsc::Receiver<StdLogMessage>,
) -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "CPUV, (A CPU Visualizer)",
        opts,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<UiApp>::new(UiApp::new(
                client_command_sender,
                engine_data_recv,
                stdlog_reciever,
            )))
        }),
    );

    return Ok(());
}
