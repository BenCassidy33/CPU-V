#![allow(warnings)]

use super::Engine;

#[derive(Debug)]
pub enum InstructionExecutionSeccess {
    Ok,
    WaitingForNextTick,
}

#[derive(Debug, PartialEq)]
pub enum InstructionExecutionError {
    EndOfLabel,
}

pub fn run_instruction(
    engine: &mut Engine,
) -> Result<InstructionExecutionSeccess, InstructionExecutionError> {
    if engine.state.tick % engine.options.engine.ipt != 0 {
        return Ok(InstructionExecutionSeccess::WaitingForNextTick);
    }

    let instructions = &engine
        .current_label
        .as_ref()
        .unwrap()
        .instructions
        .as_ref()
        .unwrap();

    if instructions.len() <= engine.state.instruction_ptr {
        return Err(InstructionExecutionError::EndOfLabel);
    }

    let instruction = &instructions[engine.state.instruction_ptr];
    println!("{:?}", instruction);

    match instruction.ty {
        _ => todo!(),
    }
    engine.state.instruction_ptr += 1;
    return Ok(InstructionExecutionSeccess::Ok);
}

pub fn call_external_function(function_name: String, args: Vec<String>) {}
