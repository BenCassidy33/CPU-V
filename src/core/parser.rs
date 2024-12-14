use std::{any::Any, fmt::write, process::exit};

use crate::core::types::*;
use std::str::FromStr;

// TODO: add in label and data parsing
pub fn parse_file(file: String) -> Program {
    let mut program = Program::new();
    let f = file.split("\n").collect::<Vec<&str>>();

    let mut skip = 0;

    for (line_num, line) in f.clone().into_iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if line.contains(".section") {
            let (_, section_name) = line.split_once(" ").unwrap();

            match section_name {
                ".data:" => {
                    println!("Found data");
                    let (variables, s) = parse_variables(&f[line_num + 1..]);
                    program.data = variables;
                    skip = s;
                    continue;
                }

                ".program:" => {
                    continue;
                }

                _ => {
                    panic!(
                        "Invalid Section Name: \"{}\"!. Perhaps you were trying to create a _label?",
                        section_name
                    )
                }
            }

            continue;
        }

        if line.contains("_") && !line.contains("JMP") {
            let label = line.trim();
            let result = parse_label(&f[line_num + 1..]);

            skip = result.1;

            if program.labels.is_none() {
                program.labels = Some(Vec::new());
            }

            if let Some(ref mut labels) = program.labels {
                labels.push(Label {
                    label_name: label.to_string(),
                    instructions: Some(result.0),
                });
            }
        }
    }

    return program;
}

pub fn parse_label(label_instructions: &[&str]) -> (Vec<Instruction>, usize) {
    let mut instructions: Vec<Instruction> = Vec::new();
    //println!("{:?}", label_instructions);

    for (line_idx, line) in label_instructions.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        if line.contains("_") && !line.contains("JMP") {
            return (instructions, line_idx);
        }

        instructions.push(Instruction::from_str(line).unwrap())
    }

    return (instructions, label_instructions.len() - 1);
}

pub fn parse_variables(variable_label: &[&str]) -> (Vec<Variable>, usize) {
    let mut variables: Vec<Variable> = Vec::new();

    for (line_idx, line) in variable_label.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let var_info = line.trim().splitn(3, " ").collect::<Vec<&str>>();
        if line.contains(".section") {
            return (variables, line_idx);
        }

        variables.push(Variable {
            ty: DataType::from_str(var_info[0]).unwrap(),
            name: var_info[1].to_string(),
            value: var_info[2].to_string(),
        })
    }

    return (variables, variable_label.len());
}
