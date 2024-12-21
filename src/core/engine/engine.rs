use std::fmt::{self, Display};
use std::sync::mpsc;
use std::thread::{self};
use std::time::{self, Duration};

use eframe::run_simple_native;
use egui::TextBuffer;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::core::lang::{parse_input, Label, Program, Registers};

use super::runner::{run_instruction, InstructionExecutionError};

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

type Payload = Option<String>;

trait PayloadImpl {
    fn extract(self) -> String;
}

impl PayloadImpl for Payload {
    fn extract(self) -> String {
        return self.unwrap_or("ERROR: Payload must not be none!".to_string());
    }
}

pub struct Engine {
    pub program: Option<Program>,
    heap_memory: Vec<i8>,
    registers: Registers,

    engine_data_sender: mpsc::Sender<EngineData>,
    client_command_reciever: mpsc::Receiver<ClientCommands>,
    stdout_sender: mpsc::Sender<StdLogMessage>,

    pub options: EngineOptions,
    pub state: EngineState,

    pub current_label: Option<Label>,
}

pub struct EngineState {
    pub tick: usize,
    pub instruction_ptr: usize,
    running_state: EngineRunningState,
}

#[derive(Debug)]
pub struct StdLogMessage {
    pub message: String,
    pub log_level: StdLogLevel,
}

#[derive(Debug, PartialEq)]
pub enum StdLogLevel {
    INFO,
    WARN,
    ERROR,

    UserPrint,
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize)]
pub enum EngineRunningState {
    #[default]
    Stopped,
    Running,
    Paused,
}

pub struct EngineOptions {
    pub memory_size: usize,
    pub ticks_per_second: usize,
    pub instructions_per_tick: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngineData {
    #[serde(rename = "Tick")]
    pub tick: usize,
    #[serde(rename = "Current State")]
    pub engine_running_state: EngineRunningState,

    #[serde(rename = "Parsing Result")]
    pub program: String,
    #[serde(rename = "IR Repserentation")]
    pub ir_repsersentation: String,

    pub responding_to: Option<ClientCommandType>,

    pub registers: Registers,
}

impl Default for EngineData {
    fn default() -> Self {
        Self {
            tick: Default::default(),
            engine_running_state: Default::default(),
            program: Default::default(),
            ir_repsersentation: "TODO".to_string(),
            responding_to: Default::default(),
            registers: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct ClientCommands {
    pub command_type: ClientCommandType,
    pub payload: Payload,
}

#[derive(Debug, EnumIter, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClientCommandType {
    Start,
    Stop,
    Pause,

    ParseFile,
    ParseWithoutUpdate,

    TranslateToIR,
    TranslateToIRWithoutUpdate,
}

impl Engine {
    pub fn new(
        options: EngineOptions,
    ) -> (
        Self,
        mpsc::Sender<ClientCommands>,
        mpsc::Receiver<EngineData>,
        mpsc::Receiver<StdLogMessage>,
    ) {
        let (data_send, data_recv) = mpsc::channel::<EngineData>();
        let (client_send, client_recv) = mpsc::channel::<ClientCommands>();
        let (log_send, log_recv) = mpsc::channel::<StdLogMessage>();

        let engine = Self {
            program: None,
            current_label: None,
            heap_memory: Vec::with_capacity(options.memory_size),
            registers: Registers {
                ..Default::default()
            },
            options,

            engine_data_sender: data_send,
            client_command_reciever: client_recv,
            stdout_sender: log_send,

            state: EngineState {
                tick: 0,
                instruction_ptr: 0,
                running_state: EngineRunningState::Stopped,
            },
        };

        return (engine, client_send, data_recv, log_recv);
    }

    pub fn get_current_state(&self, responding_to: Option<ClientCommandType>) -> EngineData {
        return EngineData {
            tick: self.state.tick,
            engine_running_state: self.state.running_state.clone(),
            program: format!("{:#?}", self.program).to_string(),
            ir_repsersentation: "TODO".to_string(),
            responding_to,
            registers: self.registers.clone(),
        };
    }

    fn send_stdlog(&self, log_level: StdLogLevel, message: &str) {
        let message = format!(
            "[{:?}] {:?} - {}",
            log_level,
            chrono::Local::now().format(TIME_FORMAT).to_string(),
            message
        );
        let log_res = self
            .stdout_sender
            .send(StdLogMessage { message, log_level });
    }

    // TODO: Fix bug where parsing result is sent but overwritted when engine is not in started
    // state
    pub fn run(mut self) {
        self.send_stdlog(StdLogLevel::INFO, "Initalizing Engine...");
        let start_time = time::Instant::now();

        loop {
            if let Ok(client_command) = self.client_command_reciever.try_recv() {
                self.run_client_commands(client_command);
            }

            if self.state.running_state != EngineRunningState::Running {
                continue;
            }

            // THIS NEEDS TO STAY HERE FOR THIS TO WORK!!!
            let send_result = self.engine_data_sender.send(self.get_current_state(None));

            let instruction_execution_result = run_instruction(&mut self);
            match instruction_execution_result {
                Err(e) => {
                    if e != InstructionExecutionError::EndOfLabel {
                        self.state.running_state = EngineRunningState::Stopped;
                        self.send_stdlog(
                            StdLogLevel::ERROR,
                            format!("Instruction Failed! {:?}", e).as_str(),
                        );
                    } else {
                        self.state.running_state = EngineRunningState::Stopped;
                        self.send_stdlog(
                            StdLogLevel::INFO,
                            format!("Program Exited With Success!").as_str(),
                        );
                    }
                }

                _ => {}
            }

            self.state.tick += 1;
            thread::sleep(Duration::from_millis(
                (1000 / self.options.ticks_per_second).try_into().unwrap(),
            ));
        }
    }

    pub fn run_client_commands(&mut self, client_command: ClientCommands) {
        match client_command.command_type {
            ClientCommandType::Start => {
                if self.state.running_state == EngineRunningState::Stopped {
                    self.state.tick = 0;
                    self.state.instruction_ptr = 0;
                    self.state.running_state = EngineRunningState::Running;
                    let program = parse_input(client_command.payload.unwrap());

                    self.current_label = program.get_start_label().ok();
                    self.program = Some(program);
                    let send_res = self
                        .engine_data_sender
                        .send(self.get_current_state(Some(ClientCommandType::Start)));
                } else {
                    self.state.running_state = EngineRunningState::Running;
                }
            }

            ClientCommandType::Pause => {
                self.state.running_state = EngineRunningState::Paused;
                self.engine_data_sender
                    .send(self.get_current_state(Some(ClientCommandType::Pause)));
            }

            ClientCommandType::Stop => {
                self.state.running_state = EngineRunningState::Stopped;
                self.engine_data_sender
                    .send(self.get_current_state(Some(ClientCommandType::Stop)));
            }

            ClientCommandType::ParseFile => {
                let payload = client_command.payload.extract();
                let program = parse_input(payload);
                self.program = Some(program);
                self.engine_data_sender
                    .send(self.get_current_state(Some(ClientCommandType::ParseFile)));
            }

            ClientCommandType::ParseWithoutUpdate => {
                let payload = client_command.payload.extract();
                let program = parse_input(payload);
                let mut current_state =
                    self.get_current_state(Some(ClientCommandType::ParseWithoutUpdate));

                current_state.program = format!("{:#?}", program).to_string();
                self.engine_data_sender.send(current_state);
            }

            ClientCommandType::TranslateToIR => todo!(),
            ClientCommandType::TranslateToIRWithoutUpdate => todo!(),
        }
    }
}
