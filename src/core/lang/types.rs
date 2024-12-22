use std::str::FromStr;

use egui::TextBuffer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Program {
    pub extern_functions: Option<Vec<String>>,
    pub data: Vec<Variable>,
    pub labels: Option<Vec<Label>>,
}

impl Program {
    pub fn new() -> Self {
        return Program {
            extern_functions: None,
            data: Vec::new(),
            labels: None,
            //env: None,
        };
    }

    pub fn get_start_label(&self) -> Result<Label, ()> {
        for label in self.labels.as_ref().unwrap() {
            if label.label_name.contains("start") {
                return Ok(label.clone());
            }
        }

        return Err(());
    }
}

#[derive(Debug)]
pub struct Data {}
#[derive(Debug)]
pub struct Env {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Label {
    pub label_name: String,
    pub instructions: Option<Vec<Instruction>>,
}
pub mod DataSizes {
    pub type Byte = u8;
    pub type SByte = i8;
    pub type Word = u16;
    pub type SWord = i16;
    pub type DWord = u32;
    pub type SDWord = i32;
    pub type QWord = u64;
    pub type TByte = [u8; 10];
    pub type Real4 = f32;
    pub type Real8 = f64;

    pub type Str4 = [char; 4];
    pub type Str8 = [char; 8];
    pub type Str16 = [char; 16];
    pub type Str32 = [char; 32];
    pub type Str64 = [char; 64];
    pub type Str128 = [char; 128];
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Byte,
    SByte,
    TByte,

    Word,
    SWord,
    DWord,
    SDWord,
    QWord,

    Real4,
    Real8,

    Str4,
    Str8,
    Str16,
    Str32,
    Str64,
    Str128,
}

#[derive(Debug, Clone)]
pub struct DataTypeInfo {}

impl FromStr for DataType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "byte" => Ok(DataType::Byte),
            "sbyte" => Ok(DataType::SByte),
            "word" => Ok(DataType::Word),
            "sword" => Ok(DataType::SWord),
            "dword" => Ok(DataType::DWord),
            "sdword" => Ok(DataType::SDWord),
            "qword" => Ok(DataType::QWord),
            "tbyte" => Ok(DataType::TByte),
            "real4" => Ok(DataType::Real4),
            "real8" => Ok(DataType::Real8),
            "str4" => Ok(DataType::Str4),
            "str8" => Ok(DataType::Str8),
            "str16" => Ok(DataType::Str16),
            "str32" => Ok(DataType::Str32),
            "str64" => Ok(DataType::Str64),
            "str128" => Ok(DataType::Str128),
            _ => Err(format!("Invalid DataType string: {}", input)),
        }
    }
}

#[derive(Debug)]
pub enum SectionError {
    InvalidSectionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Variable {
    pub name: String,
    pub ty: DataType,
    pub value: String, // TODO: Make sure that type matches value
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Instruction {
    pub ty: InstructionType,
    pub val: InstructionValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum InstructionValue {
    SingleValue(String),
    MultipleValue((String, String)),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstructionType {
    CALL,

    LOAD,
    MOVE,
    INC,
    DEC,

    ADD,
    SUB,

    CMP,
    JEQ,
    JLT,
    JGT,
    JMP,

    NOP,
    BRK,
    EXIT,
}

impl InstructionType {
    pub fn has_multiple_values(&self) -> bool {
        return match self {
            Self::JMP => true,
            _ => false,
        };
    }

    pub fn is_valueless(&self) -> bool {
        return match self {
            Self::NOP => true,
            _ => false,
        };
    }
}

impl FromStr for InstructionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "call" => Ok(InstructionType::CALL),
            "load" => Ok(InstructionType::LOAD),
            "move" => Ok(InstructionType::MOVE),
            "inc" => Ok(InstructionType::INC),
            "dec" => Ok(InstructionType::DEC),
            "add" => Ok(InstructionType::ADD),
            "sub" => Ok(InstructionType::SUB),
            "cmp" => Ok(InstructionType::CMP),
            "jeq" => Ok(InstructionType::JEQ),
            "jlt" => Ok(InstructionType::JLT),
            "jgt" => Ok(InstructionType::JGT),
            "jmp" => Ok(InstructionType::JMP),
            "nop" => Ok(InstructionType::NOP),
            "brk" => Ok(InstructionType::BRK),
            "exit" => Ok(InstructionType::EXIT),
            _ => Err(format!("Unknown instruction: {}", s)),
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instruction_label = String::new();
        let mut value = String::new();

        if let Some((left, right)) = s.trim().split_once(" ") {
            instruction_label = left.to_string();
            value = right.to_string();
        } else {
            instruction_label = s.trim().to_string();

            if !InstructionType::from_str(&instruction_label.to_string())?.is_valueless() {
                return Err(format!(
                    "Error: Instruction is not valueless: {}, {:?}",
                    instruction_label,
                    InstructionType::from_str(&instruction_label.to_string()).unwrap()
                ));
            };
        }

        let ty = InstructionType::from_str(&instruction_label)?;
        let mut values: Option<(&str, &str)> = None;
        if ty.has_multiple_values() {
            values = value.split_once(",");
        }

        return Ok(Self {
            ty,
            val: match values {
                Some(val) => {
                    let left = val.0.to_string();
                    let right = val.1.to_string();
                    InstructionValue::MultipleValue((left, right))
                }
                None => InstructionValue::SingleValue(value.to_string()),
            },
        });
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Registers {
    /// 64 bit registers
    pub rax: u64, // accumulator
    pub rbx: u64, // base register
    pub rcx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,

    /// 32 bit registers
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub edi: u32,
    pub esi: u32,
    pub edx: u32,

    /// 16 bit registers
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub sp: u16,
    pub bp: u16,
    pub di: u16,
    pub si: u16,
    pub dx: u16,

    /// 8 bit registers
    pub ah: u8,
    pub al: u8,
    pub bh: u8,
    pub bl: u8,
    pub ch: u8,
    pub cl: u8,
    pub spl: u8,
    pub bpl: u8,
    pub dil: u8,
    pub sil: u8,
    pub dh: u8,
    pub dl: u8,

    /// 64 bit signed registers
    pub srax: i64, // accumulator
    pub srbx: i64, // base register
    pub srcx: i64,
    pub srsp: i64,
    pub srbp: i64,
    pub srdi: i64,
    pub srsi: i64,
    pub srdx: i64,

    /// 32 bit signed registers
    pub seax: i32,
    pub sebx: i32,
    pub secx: i32,
    pub sesp: i32,
    pub sebp: i32,
    pub sedi: i32,
    pub sesi: i32,
    pub sedx: i32,

    /// 16 bit signed registers
    pub sax: i16,
    pub sbx: i16,
    pub scx: i16,
    pub ssp: i16,
    pub sbp: i16,
    pub sdi: i16,
    pub ssi: i16,
    pub sdx: i16,

    /// 8 bit signed registers
    pub sah: i8,
    pub sal: i8,
    pub sbh: i8,
    pub sbl: i8,
    pub sch: i8,
    pub scl: i8,
    pub sspl: i8,
    pub sbpl: i8,
    pub sdil: i8,
    pub ssil: i8,
    pub sdh: i8,
    pub sdl: i8,
}
