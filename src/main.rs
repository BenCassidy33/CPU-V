//#![allow(warnings, unused)]
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

const FPS: u64 = 60;

fn main() {
    env_logger::init();

    let options = EngineOptions {
        memory_size: 2048,
        ticks_per_second: 1,
    };

    let (engine, engine_data_reciever, client_command_sender) = Engine::new(options);

    thread::spawn(move || {
        engine.run();
    });

    ui::window::init(engine_data_reciever, client_command_sender);
}
