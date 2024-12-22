use crate::core::IR::{DataType, Instruction, InstructionType, Label, Program, Variable};
use std::str::FromStr;

// TODO: add in label and data parsing
pub fn parse_input(file: String) -> Program {
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
                    let (variables, s) = parse_variables(&f[line_num + 1..]);
                    program.static_variables = variables;
                    skip = s;
                    continue;
                }

                ".extern" => {
                    let (functions, s) = parse_extern(&f[line_num + 1..]);
                    program.extern_functions = Some(functions);
                    skip = s;
                    continue;
                }

                ".program:" => {
                    continue;
                }

                _ => {
                    panic!(
                        "Invalid Section Name: \"{}\"!. Perhaps you were trying to create a @label?",
                        section_name
                    )
                }
            }

            continue;
        }

        if line.contains("@") && !has_jump(&line) {
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

pub fn has_jump(line: &&str) -> bool {
    return line.contains("JMP")
        || line.contains("JEQ")
        || line.contains("JLT")
        || line.contains("JGT");
}

pub fn parse_extern(label_functions: &[&str]) -> (Vec<String>, usize) {
    let mut functions: Vec<String> = Vec::new();

    for (line_idx, line) in label_functions.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        if line.contains(".section") {
            return (functions, line_idx);
        }

        functions.push(line.trim().to_string());
    }

    return (functions, label_functions.len() - 1);
}

pub fn parse_label(label_instructions: &[&str]) -> (Vec<Instruction>, usize) {
    let mut instructions: Vec<Instruction> = Vec::new();

    for (line_idx, line) in label_instructions.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        if line.contains("@") && !has_jump(line) {
            return (instructions, line_idx);
        }

        if line.contains(".section") {
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
            inital_value: var_info[2].to_string(),
        })
    }

    return (variables, variable_label.len());
}
