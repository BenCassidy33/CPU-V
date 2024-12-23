mod config;
mod core;
mod ui;

use core::engine::Engine;

use clap::Parser;
use config::RootConfig;
use std::{process::exit, thread};

const FPS: u64 = 60;

fn main() {
    let args = CliArgs::parse();
    let options = parse_config(args.config_path);

    let (engine, client_command_sender, engine_data_reciever, stdlog_reciever) =
        Engine::new(options);

    thread::spawn(move || {
        engine.run();
    });

    let _ = ui::window::init(client_command_sender, engine_data_reciever, stdlog_reciever);
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArgs {
    #[arg(short, long)]
    config_path: Option<String>,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    write_out: Option<bool>,
}

pub fn parse_config(config_path: Option<String>) -> RootConfig {
    let mut config = RootConfig::default();
    let home_config = std::fs::read_to_string("~/.config/cpuv/config.toml");

    if config_path.is_none() && home_config.is_err() {
        println!("Config not specified, Defaulting.",);
        return config;
    } else if home_config.is_ok() {
        let tmp = toml::from_str::<config::RootConfig>(&home_config.unwrap());
        if let Err(ref e) = tmp {
            eprintln!("{}", e);
            exit(1);
        }
        config = tmp.unwrap();
    } else if config_path.is_some() {
        let config_path = config_path.clone().unwrap();
        let raw = std::fs::read_to_string(&config_path);

        if raw.is_err() {
            eprintln!("Config file not found at {:?}. Exiting...", config_path);
            exit(1);
        }

        let tmp = toml::from_str::<config::RootConfig>(&raw.unwrap());
        if let Err(ref e) = tmp {
            eprintln!("{}", e);
            exit(1);
        }

        config = tmp.unwrap();
    };

    config
}
