use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use super::widgets::render_info_and_controls;
use crate::{
    core::engine::{ClientCommands, EngineData},
    FPS,
};

pub struct UiApp {
    pub data_recv: mpsc::Receiver<EngineData>,
    pub command_sender: mpsc::Sender<Vec<ClientCommands>>,

    pub sidebar_shown: bool,

    pub previous_data: EngineData,
}

impl UiApp {
    pub fn new(
        data_recv: mpsc::Receiver<EngineData>,
        command_sender: mpsc::Sender<Vec<ClientCommands>>,
    ) -> Self {
        return Self {
            data_recv,
            command_sender,
            sidebar_shown: true,
            previous_data: EngineData::default(),
        };
    }
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(1000 / FPS));

        //let window = egui::Window::new("Engine Data").show(ctx, |ui| {
        //});

        render_info_and_controls(self, ctx);
    }
}
