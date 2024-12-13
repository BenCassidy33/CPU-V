#![allow(warnings, unused)]

mod core;

use crate::core::core::parse_file;
use std::{env, fs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_buf: String = String::new();

    for arg in args {
        if arg.ends_with(".asm") {
            file_buf = fs::read_to_string(arg).expect("Could not read file. Dump");
        }
    }

    parse_file(file_buf);
}
