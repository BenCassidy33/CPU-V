use eframe::egui;
use std::sync::{Arc, Mutex};
use std::{fmt::format, sync::mpsc};

use crate::core::engine::{ClientCommands, EngineData};

static WINDOW_WIDTH: f32 = 2560.0;
static WINDOW_HEIGHT: f32 = 1440.0;

//use super::widgets::render_engine_data;
//pub fn init(
//    engine_data_reciever: Arc<Mutex<mpsc::Receiver<EngineData>>>,
//    client_command_sender: mpsc::Sender<ClientCommands>,
//) -> eframe::Result {
//    let opts = eframe::NativeOptions {
//        viewport: egui::ViewportBuilder::default().with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
//        ..Default::default()
//    };
//
//    eframe::run_native(
//        "CPUV, (A CPU Visualizer)",
//        opts,
//        Box::new(|cc| {
//            egui_extras::install_image_loaders(&cc.egui_ctx);
//            Ok(Box::<App>::new(App::new(
//                engine_data_reciever,
//                client_command_sender,
//            )))
//        }),
//    );
//
//    return Ok(());
//}
//
//pub struct App {
//    pub app_name: String,
//
//    pub engine_data_reciever: Arc<Mutex<mpsc::Receiver<EngineData>>>,
//    pub client_command_sender: mpsc::Sender<ClientCommands>,
//}
//
//impl App {
//    pub fn new(
//        engine_data_reciever: Arc<Mutex<mpsc::Receiver<EngineData>>>,
//        client_command_sender: mpsc::Sender<ClientCommands>,
//    ) -> Self {
//        return Self {
//            app_name: "App".to_string(),
//            engine_data_reciever,
//            client_command_sender,
//        };
//    }
//}
//
//impl eframe::App for App {
//    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//        render_engine_data(self, ctx);
//    }
//}
