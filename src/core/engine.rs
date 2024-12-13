use std::sync::mpsc;
use std::thread;

pub struct Engine {
    /// the amount time the engine pauses between steps
    pub step_time: usize,
    /// The amount of steps that the program takes per interval of time (step_time)
    pub steps_per_interval: usize,
    // TODO: change this to send the correct data
    pub client_receiver: mpsc::Receiver<EngineData>,
    pub client_sender: mpsc::Sender<ClientData>,

    step: usize,
    // TODO: change this to send the correct data
    engine_receiver: mpsc::Receiver<ClientData>,
    engine_sender: mpsc::Sender<EngineData>,
}

/// Data sent to the engine, typically settings
pub struct ClientData {}

/// Data sent to the client about the engine and its state
pub struct EngineData {
    pub cpu_data: CPU,
    pub step: usize,
    pub step_time: usize,
    pub steps_per_interval: usize,
    pub steps_per_second: usize,
}

pub struct CPU {}
