use core::time;
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
    /// Handler for the client to recieve data from the engine
    pub engine_data_receiver: mpsc::Receiver<EngineData>,
    /// Handler for the engine to recieve commands from the client
    pub client_command_sender: mpsc::Sender<ClientCommands>,

    line: usize,
    /// handler for the engine to send data to the client
    engine_data_sender: mpsc::Sender<EngineData>,
    /// Handler for the client to send commands to the engine
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

            /// Handler for the client to recieve data from the engine
            engine_data_receiver: eng_recv,
            /// Handler for the client to send commands to the engine
            client_command_sender: opt_sender,
            /// Handler for the engine to recieve commands from the client
            client_command_receiver: opt_recv,
            /// handler for the engine to send data to the client
            engine_data_sender: eng_sender,
        };
    }

    /// Just a pretty function to start the main_loop
    pub fn start(mut self) {
        self.engine_running_state = EngineRunningState::Running;
        self.main_loop().join().unwrap();
    }

    fn main_loop(mut self) -> JoinHandle<()> {
        let mut tick = 0;

        let mut start_tick = 0;
        let mut start_tick_time = Instant::now();
        let mut avg_ticks = 0;

        return thread::spawn(move || loop {
            let start_time = Instant::now();

            //println!(
            //    "Running!\n{:#?},\n time between ticks: {:#?}",
            //    self.options,
            //    time::Duration::from_secs(self.options.ticks_per_second as u64)
            //);

            if let Ok(client_commands) = self.client_command_receiver.try_recv() {
                self.run_client_commands(client_commands);
            };

            let end_time = Instant::now();
            let dt = end_time - start_time;

            if ((end_time - start_tick_time).as_millis() >= self.options.time_between_reports) {
                avg_ticks = (tick - start_tick) / (Instant::now() - start_tick_time).as_secs();
                start_tick_time = Instant::now();
                start_tick = tick;
                let report = self.generate_report(tick, dt, avg_ticks);
                println!("{:#?}", report);
                self.engine_data_sender.send(report);
            }

            if self.engine_running_state == EngineRunningState::Running {
                if tick.checked_rem(self.options.lines_per_tick.try_into().unwrap()) == Some(0) {
                    self.step_tick();
                }

                tick += 1
            };

            thread::sleep(time::Duration::from_millis(
                1000 / self.options.ticks_per_second as u64,
            ));
        });
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

    pub fn generate_report(&self, tick: u64, dt: time::Duration, avg_ticks: u64) -> EngineData {
        return EngineData {
            tick,
            last_tick_time: dt.as_millis(),
            lines_per_tick: self.options.lines_per_tick,
            estimated_ticks_per_second: avg_ticks,
        };
    }
}
