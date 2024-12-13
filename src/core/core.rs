use std::{fmt::write, process::exit};

use crate::core::types::*;
use std::str::FromStr;

// TODO: add in label and data parsing
pub fn parse_file(file: String) {
    let mut sections: Vec<Section> = Vec::new();
    let mut section_num = 0;

    for (line_num, line) in file.split("\n").enumerate() {
        if line.contains(".section") {
            let (section_delim, section_name) = line.split_once(" ").unwrap();
            sections.push(match section_name {
                ".program:" => Section::Program(ProgramLabel {
                    label_name: "program".to_string(),
                    instructions: None,
                }),
                ".data:" => Section::Data(None),
                _ => {
                    eprintln!("Invalid Setion Type at Line: {}", line_num);
                    exit(1);
                }
            });

            section_num += 1;
            continue;
        }

        if line.contains(":") {
            continue;
        }

        let Some((instruction, value)) = line.trim().split_once(" ") else {
            continue;
        };

        let instruction_type = InstructionType::from_str(instruction);
        if instruction_type.is_ok() {
            let section = &mut sections[section_num - 1];

            if let Section::Program(program) = section {
                if program.instructions.is_none() {
                    program.instructions = Some(Vec::new());
                }

                let instruct = Instruction {
                    flags: instruction_type.as_ref().unwrap().flags(),
                    ty: instruction_type.unwrap(),
                    val: value.to_string(),
                };

                program.instructions.as_mut().unwrap().push(instruct);
            }
        }
    }

    println!("{:#?}", sections);
}
