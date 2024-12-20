#![allow(warnings, unused)]
#![feature(deadline_api)]

mod core;
mod ui;

use core::engine::{Engine, EngineOptions};
use std::time::Duration;
use std::{
    env, fs,
    io::Read,
    sync::{Arc, Mutex},
};
use std::{thread, time};

use crate::core::parser::parse_file;

const FPS: u64 = 60;

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
        memory_size: 2048,
        ticks_per_second: 1,
    };

    let mut data_records: Arc<Mutex<Vec<core::engine::EngineData>>> =
        Arc::new(Mutex::new(Vec::new()));

    let (engine, engine_data_reciever, client_command_sender) = Engine::new(options);

    thread::spawn(move || {
        engine.start();
    });

    ui::window::init(engine_data_reciever, client_command_sender);
}
