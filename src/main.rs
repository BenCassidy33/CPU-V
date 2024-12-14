#![allow(warnings, unused)]

mod core;

use crate::core::parser::parse_file;
use core::engine::{Engine, EngineOptions};
use std::{env, fs, io::Read};

fn main() {
    let mut file_buf: String = String::new();

    for arg in env::args() {
        if arg.ends_with(".asm") {
            file_buf = fs::read_to_string(arg).expect("Could not read file. Process Error");
        }
    }

    let program = parse_file(file_buf);
    let mut options = EngineOptions::default();
    options.ticks_per_second = 500;
    options.time_between_reports = 5000;
    options.lines_per_tick = 50;
    let engine = Engine::new(options, program);

    engine.start()
}
