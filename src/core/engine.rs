use std::sync::mpsc;
use std::thread::{self};
use std::time::{self, Duration};

use structmap::ToMap;
use structmap_derive::ToMap;

use super::parser::parse_input;
use super::types::{Program, Registers};

pub struct Engine {
    program: Option<Program>,
    program_memory: Vec<i32>,
    registers: Registers,

    engine_data_sender: mpsc::Sender<EngineData>,
    client_command_reciever: mpsc::Receiver<ClientCommands>,

    options: EngineOptions,
    state: EngineState,
}

pub struct EngineState {
    tick: usize,
    instruction_ptr: usize,
    running_state: EngineRunningState,
}

#[derive(PartialEq)]
pub enum EngineRunningState {
    Running,
    Stopped,
    Paused,
}

pub struct EngineOptions {
    pub memory_size: usize,
    pub ticks_per_second: usize,
}

#[derive(Debug, ToMap, Default, Clone)]
pub struct EngineData {
    tick: usize,
}

pub struct ClientCommands {
    pub command_type: ClientCommandType,
    pub payload: Option<String>,
}

pub enum ClientCommandType {
    Start,
    Stop,
    Pause,
}

impl Engine {
    pub fn new(
        options: EngineOptions,
    ) -> (
        Self,
        mpsc::Receiver<EngineData>,
        mpsc::Sender<ClientCommands>,
    ) {
        let (data_send, data_recv) = mpsc::channel::<EngineData>();
        let (client_send, client_recv) = mpsc::channel::<ClientCommands>();

        let engine = Self {
            program: None,
            program_memory: Vec::with_capacity(options.memory_size),
            registers: Registers {
                ..Default::default()
            },
            options,

            engine_data_sender: data_send,
            client_command_reciever: client_recv,

            state: EngineState {
                tick: 0,
                instruction_ptr: 0,
                running_state: EngineRunningState::Stopped,
            },
        };

        return (engine, data_recv, client_send);
    }

    fn send_error(error_msg: String) {
        todo!("Need to implmient sending errors")
    }

    pub fn start(mut self, input: String) {
        if input.is_empty() && self.program.is_none() {
            Self::send_error("Can not run program with an empty file!".to_string());
        }
        self.program = Some(parse_input(input));
        self.run();
    }

    pub fn run(mut self) {
        let start_time = time::Instant::now();

        loop {
            if let Ok(client_command) = self.client_command_reciever.try_recv() {
                match client_command.command_type {
                    ClientCommandType::Start => {
                        if self.state.running_state == EngineRunningState::Stopped {
                            self.state.tick = 0;
                            self.state.instruction_ptr = 0;
                            self.state.running_state = EngineRunningState::Running;
                            self.program = Some(parse_input(client_command.payload.unwrap()));
                        } else {
                            self.state.running_state = EngineRunningState::Running;
                        }
                    }
                    ClientCommandType::Pause => {
                        self.state.running_state = EngineRunningState::Paused
                    }
                    ClientCommandType::Stop => {
                        self.state.running_state = EngineRunningState::Stopped
                    }
                }
            }

            if self.state.running_state != EngineRunningState::Running {
                continue;
            }

            let send_result = self.engine_data_sender.send(EngineData {
                tick: self.state.tick,
            });

            self.state.tick += 1;
            thread::sleep(Duration::from_millis(
                (1000 / self.options.ticks_per_second).try_into().unwrap(),
            ));
        }
    }

    pub fn pause() {}
    pub fn stop() {}
}
