use core::time;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use std::{os::unix::thread::JoinHandleExt, sync::mpsc};

use super::types::{self, Label, Program, Registers};

#[derive(Debug)]
pub struct EngineOptions {
    /// the amount time the engine pauses between steps (counted in ms)
    pub ticks_per_second: usize,
    /// The amount of steps that the program takes per interval of time (step_time)
    pub lines_per_tick: usize,
    /// Determines how many times engine data will be sent to the client per tick;
    pub data_updates_per_tick: usize,
    /// Determines the amount of time spend between generating and sending reports
    pub time_between_reports: u128,
}

/// Data sent to the engine, typically settings
#[derive(Clone, Debug)]
pub enum ClientCommands {
    Start,
    Stop,
    Pause,
    StepForward,
    StepBackward,
}

pub enum ClientCommandError {
    AlreadyRunning,
    AlreadyStopped,
    AlreadyPaused,
}

/// Data sent to the client about the engine and its state
#[derive(Debug)]
pub struct EngineData {
    pub tick: u64,
    pub last_tick_time: u128,
    pub lines_per_tick: usize,
    pub estimated_ticks_per_second: u64,
}

impl Default for EngineOptions {
    fn default() -> Self {
        return Self {
            ticks_per_second: 5,
            lines_per_tick: 1,
            data_updates_per_tick: 1,
            time_between_reports: 1000,
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
    pub options: EngineOptions,

    line: usize,
    registers: Registers,
    program: Program,
    label: Option<Label>,

    pub tick_data: EngineTickData,
    pub client_commands: Option<Vec<ClientCommands>>,

    condvar: Arc<(Mutex<bool>, Condvar)>,
}

#[derive(Debug)]
pub struct EngineTickData {
    // commands and info that were ran and generated on the last tick
    pub last_client_commands: Option<Vec<ClientCommands>>,
    pub last_generated_report: Option<EngineData>,
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

            tick_data: EngineTickData {
                last_client_commands: None,
                last_generated_report: None,
            },

            client_commands: None,

            condvar: Arc::new((Mutex::new(true), Condvar::new())),
        };
    }

    // TODO: Update this so that it works for running tick by tick
    pub fn run_simulation_tick(&mut self) {
        println!("Engine is Running!");
        let mut tick = 0;

        let mut start_tick = 0;
        let mut start_tick_time = Instant::now();
        let mut avg_ticks = 0;

        let start_time = Instant::now();
        let end_time = Instant::now();
        let dt = end_time - start_time;

        if ((end_time - start_tick_time).as_millis() >= self.options.time_between_reports) {
            avg_ticks = (tick - start_tick) / (Instant::now() - start_tick_time).as_secs();
            start_tick_time = Instant::now();
            start_tick = tick;

            self.generate_report(tick, dt, avg_ticks);
        }

        if self.engine_running_state == EngineRunningState::Running {
            if tick.checked_rem(self.options.lines_per_tick.try_into().unwrap()) == Some(0) {}

            tick += 1
        };
    }

    pub fn run_client_commands(&mut self, client_commands: ClientCommands) {
        match client_commands {
            ClientCommands::Pause => self.engine_running_state = EngineRunningState::Paused,
            ClientCommands::Start => self.engine_running_state = EngineRunningState::Running,
            ClientCommands::Stop => self.engine_running_state = EngineRunningState::Stoppped,

            ClientCommands::StepForward => {
                todo!()
            }

            ClientCommands::StepBackward => self.step_back_tick(),
        };
    }

    pub fn step_tick(&mut self) {
        //println!("Stepping!");
        //self.update();
        //todo!()
    }

    pub fn step_back_tick(&mut self) {}
    pub fn update(&mut self) {}

    pub fn generate_report(&mut self, tick: u64, dt: time::Duration, avg_ticks: u64) {
        self.tick_data = EngineTickData {
            last_generated_report: Some(EngineData {
                tick,
                last_tick_time: dt.as_millis(),
                lines_per_tick: self.options.lines_per_tick,
                estimated_ticks_per_second: avg_ticks,
            }),

            last_client_commands: self.client_commands.clone(),
        };

        self.client_commands = None;
    }

    pub fn get_generated_engine_report(&self) -> &EngineTickData {
        return &self.tick_data;
    }
}
