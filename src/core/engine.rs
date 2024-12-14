use core::time;
use std::sync::mpsc;
use std::thread;

use super::types::{self, Label, Program, Registers};

pub struct EngineOptions {
    /// the amount time the engine pauses between steps (counted in ms)
    pub tick_sleep_time: usize,
    /// The amount of steps that the program takes per interval of time (step_time)
    pub steps_per_tick: usize,
    /// Determines how many times engine data will be sent to the client per tick;
    pub data_updates_per_tick: usize,
}

/// Data sent to the engine, typically settings
pub enum ClientCommands {
    Start,
    Stop,
    Pause,
    StepForward,
    StepBackward,
}

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
            tick_sleep_time: 100,
            steps_per_tick: 1,
            data_updates_per_tick: 1,
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum EngineRunningState {
    Running,
    Paused,
    Stoppped,
}

pub struct Engine {
    pub tick: usize,
    pub engine_running_state: EngineRunningState,
    pub engine_data_receiver: mpsc::Receiver<EngineData>,
    pub client_command_sender: mpsc::Sender<ClientCommands>,

    line: usize,
    client_data_sender: mpsc::Sender<EngineData>,
    client_command_receiver: mpsc::Receiver<ClientCommands>,

    registers: Registers,
    options: EngineOptions,
    program: Program,

    label: Option<Label>,
}

impl Engine {
    pub fn new(options: EngineOptions, program: Program) -> Self {
        let (eng_sender, eng_recv) = mpsc::channel::<EngineData>();
        let (opt_sender, opt_recv) = mpsc::channel::<ClientCommands>();

        return Self {
            options,
            program,
            registers: Registers::default(),
            label: None,
            engine_running_state: EngineRunningState::Stoppped,

            line: 0,
            tick: 0,
            engine_data_receiver: eng_recv,
            client_command_sender: opt_sender,
            client_command_receiver: opt_recv,
            client_data_sender: eng_sender,
        };
    }

    pub fn start(&mut self) {
        while self.engine_running_state == EngineRunningState::Running {
            if let Ok(client_commands) = self.client_command_receiver.recv() {
                match client_commands {
                    ClientCommands::Pause => self.engine_running_state = EngineRunningState::Paused,

                    ClientCommands::Start => {
                        self.engine_running_state = EngineRunningState::Running
                    }

                    ClientCommands::Stop => {
                        self.engine_running_state = EngineRunningState::Stoppped
                    }

                    ClientCommands::StepForward => {
                        self.step_tick();
                        continue;
                    }

                    ClientCommands::StepBackward => self.step_back_tick(),
                };
            };

            self.step_tick();

            thread::sleep(time::Duration::from_millis(
                self.options.tick_sleep_time as u64,
            ));
        }
    }

    pub fn pause(&mut self) {}
    pub fn stop(&mut self) {}

    pub fn step_tick(&mut self) {
        self.update();
        todo!()
    }

    pub fn step_back_tick(&mut self) {}
    pub fn update(&mut self) {}
}
