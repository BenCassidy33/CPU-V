#![allow(warnings, unused)]

mod core;
mod ui;

use core::engine::{Engine, EngineOptions};
use std::thread;

const FPS: u64 = 60;

//pub const POINTER_ARROW: &str = "↓";

fn main() {
    env_logger::init();

    let options = EngineOptions {
        memory_size: 2048,
        ticks_per_second: 1,
    };

    let (engine, client_command_sender, engine_data_reciever, stdlog_reciever) =
        Engine::new(options);

    thread::spawn(move || {
        engine.run();
    });

    ui::window::init(client_command_sender, engine_data_reciever, stdlog_reciever);
}
