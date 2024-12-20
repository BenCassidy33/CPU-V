use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fmt::format, sync::mpsc};

use crate::core::engine::{ClientCommandType, ClientCommands, EngineData};
use crate::ui::app::UiApp;

pub const WINDOW_WIDTH: f32 = 2560.0;
pub const WINDOW_HEIGHT: f32 = 1440.0;

pub fn init(
    engine_data_recv: mpsc::Receiver<EngineData>,
    client_command_sender: mpsc::Sender<ClientCommands>,
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
                engine_data_recv,
                client_command_sender,
            )))
        }),
    );

    return Ok(());
}
