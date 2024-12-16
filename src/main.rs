#![allow(warnings, unused)]

mod core;
mod ui;

use crate::core::parser::parse_file;
use core::engine::{Engine, EngineOptions};
use std::{env, fs, io::Read};

fn main() {
    env_logger::init();

    let mut file_buf: String = String::new();

    for arg in env::args() {
        if arg.ends_with(".asm") {
            file_buf = fs::read_to_string(arg).expect("Could not read file. Process Error");
        }
    }

    let program = parse_file(file_buf);

    let options = EngineOptions {
        ticks_per_second: 500,
        time_between_reports: 5000,
        lines_per_tick: 50,
        ..Default::default()
    };

    let mut engine = Engine::new(options, program);
    ui::window::init(engine.engine_data_receiver, engine.client_command_sender);
    engine.start();
}
