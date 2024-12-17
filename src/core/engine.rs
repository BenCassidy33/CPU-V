use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{self, Duration};

use super::types::{self, Label, Program, Registers};

pub struct Engine {
    memory: Vec<i32>,
    registers: Registers,
    engine_data_sender: mpsc::Sender<EngineData>,
    client_command_reciever: mpsc::Receiver<Vec<ClientCommands>>,

    options: EngineOptions,
    state: EngineState,
}

pub struct EngineState {
    tick: usize,
}

pub struct EngineOptions {
    pub memory_size: usize,
    pub ticks_per_second: usize,
}

#[derive(Debug)]
pub struct EngineData {
    tick: usize,
}
pub struct ClientCommands {}

impl Engine {
    pub fn new(
        options: EngineOptions,
    ) -> (
        Self,
        mpsc::Receiver<EngineData>,
        mpsc::Sender<Vec<ClientCommands>>,
    ) {
        let (data_send, data_recv) = mpsc::channel::<EngineData>();
        let (client_send, client_recv) = mpsc::channel::<Vec<ClientCommands>>();

        return (
            Self {
                memory: Vec::with_capacity(options.memory_size),
                registers: Registers {
                    ..Default::default()
                },
                options,

                engine_data_sender: data_send,
                client_command_reciever: client_recv,

                state: EngineState { tick: 0 },
            },
            data_recv,
            client_send,
        );
    }

    pub fn start(self) {
        self.main_loop().join().unwrap();
    }

    fn main_loop(mut self) -> JoinHandle<()> {
        let start_time = time::Instant::now();
        thread::spawn(move || loop {
            println!(
                "Engine is on Tick: {} (Running at {}tps) [{:?}]",
                self.state.tick,
                self.options.ticks_per_second,
                start_time.elapsed()
            );

            self.engine_data_sender.send(EngineData {
                tick: self.state.tick,
            });

            self.state.tick += 1;
            thread::sleep(Duration::from_millis(
                (1000 / self.options.ticks_per_second).try_into().unwrap(),
            ));
        })
    }

    pub fn pause() {}
    pub fn stop() {}
}
