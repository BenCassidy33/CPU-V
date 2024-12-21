#![allow(warnings, unused)]

mod core;
mod ui;

use core::{
    engine::{Engine, EngineOptions},
    lang::parse_input,
};
use std::thread;

const FPS: u64 = 60;

//pub const POINTER_ARROW: &str = "↓";
//pub const DEFAULT_FILE: &str = "./test_file.cpu";

fn main() {
    env_logger::init();

    //let test_file = std::fs::read_to_string(DEFAULT_FILE).unwrap();

    let options = EngineOptions {
        memory_size: 2048,
        ticks_per_second: 1,
        instructions_per_tick: 1,
    };

    let (engine, client_command_sender, engine_data_reciever, stdlog_reciever) =
        Engine::new(options);

    thread::spawn(move || {
        engine.run();
    });

    ui::window::init(client_command_sender, engine_data_reciever, stdlog_reciever);
}
