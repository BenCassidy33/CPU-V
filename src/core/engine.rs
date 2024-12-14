use std::sync::mpsc;
use std::thread;

use super::types::{self, Program};

pub struct EngineOptions {
    /// the amount time the engine pauses between steps (counted in ms)
    pub tick_time: usize,
    /// The amount of steps that the program takes per interval of time (step_time)
    pub steps_per_tick: usize,
    /// Determines how many times engine data will be sent to the client per tick;
    pub data_updates_per_tick: usize,
}

/// Data sent to the engine, typically settings
pub struct ClientOptions {}

/// Data sent to the client about the engine and its state
pub struct EngineData {
    pub tick: usize,
    pub tick_time: usize,
    pub steps_per_tick: usize,
    pub ticks_per_second: usize,
}

impl Default for EngineOptions {
    fn default() -> Self {
        return Self {
            tick_time: 100,
            steps_per_tick: 1,
            data_updates_per_tick: 1,
        };
    }
}

pub struct Engine {
    options: EngineOptions,
    program: Program,

    pub tick: usize,
    line: usize,

    pub engine_data_receiver: mpsc::Receiver<EngineData>,
    client_data_sender: mpsc::Sender<EngineData>,

    pub client_options_sender: mpsc::Sender<ClientOptions>,
    client_options_receiver: mpsc::Receiver<ClientOptions>,
}

impl Engine {
    pub fn new(options: EngineOptions, program: Program) -> Self {
        let (eng_sender, eng_recv) = mpsc::channel::<EngineData>();
        let (opt_sender, opt_recv) = mpsc::channel::<ClientOptions>();

        return Self {
            options,
            program,

            line: 0,
            tick: 0,
            engine_data_receiver: eng_recv,
            client_options_sender: opt_sender,
            client_options_receiver: opt_recv,
            client_data_sender: eng_sender,
        };
    }

    pub fn start() {}
    pub fn pause() {}
    pub fn stop() {}

    pub fn start_tick() {}
}
