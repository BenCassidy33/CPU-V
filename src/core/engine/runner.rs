use super::Engine;
use crate::core::lang::Instruction;
use crate::core::lang::InstructionType;

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
    if engine.state.tick % engine.options.instructions_per_tick != 0 {
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

    //match instruction.ty {
    //    InstructionType::CALL => todo!(),
    //    InstructionType::LOAD => todo!(),
    //    InstructionType::MOVE => todo!(),
    //    InstructionType::INC => todo!(),
    //    InstructionType::DEC => todo!(),
    //    InstructionType::ADD => todo!(),
    //    InstructionType::SUB => todo!(),
    //    InstructionType::CMP => todo!(),
    //    InstructionType::JEQ => todo!(),
    //    InstructionType::JLT => todo!(),
    //    InstructionType::JGT => todo!(),
    //    InstructionType::JMP => todo!(),
    //    InstructionType::NOP => todo!(),
    //    InstructionType::BRK => todo!(),
    //    InstructionType::EXIT => todo!(),
    //}
    //
    engine.state.instruction_ptr += 1;
    return Ok(InstructionExecutionSeccess::Ok);
}
