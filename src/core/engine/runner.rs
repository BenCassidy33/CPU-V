use super::Engine;
use crate::core::lang::Instruction;

pub fn run_instruction(engine: &mut Engine) {
    let instruction = &engine.current_label.as_ref().unwrap().instructions;
    //    .clone()
    //    .unwrap()[engine.label_instruction_pointer];
    //println!("{:?}", instruction);
    //match instruction {
    //    _ => todo!(""),
    //}
}
