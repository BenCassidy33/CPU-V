use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub enum Section {
    Label(ProgramLabel),
    Data(Option<Variable>),
    Env,
}

#[derive(Debug, Clone)]
pub struct ProgramLabel {
    pub label_name: String,
    pub instructions: Option<Vec<Instruction>>,
}

pub mod DataTypes {
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

#[derive(Debug, Clone)]
pub enum DataType {
    Byte(DataTypes::Byte),
    SByte(DataTypes::SByte),
    Word(DataTypes::Word),
    SWord(DataTypes::SWord),
    DWord(DataTypes::DWord),
    SDWord(DataTypes::SDWord),
    QWord(DataTypes::QWord),
    TByte(DataTypes::TByte),
    Real4(DataTypes::Real4),
    Real8(DataTypes::Real8),
    Str4(DataTypes::Str4),
    Str8(DataTypes::Str8),
    Str16(DataTypes::Str16),
    Str32(DataTypes::Str32),
    Str64(DataTypes::Str64),
    Str128(DataTypes::Str128),
}

#[derive(Debug)]
pub enum SectionError {
    InvalidSectionType,
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    ty: DataType,
    value: DataType,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub ty: InstructionType,
    pub val: String,
    pub flags: Vec<Flag>,
}

#[derive(Debug, Clone)]
pub enum InstructionType {
    // Load/Store Operations
    LDA, // Sets: N, Z
    LDX, // Sets: N, Z
    LDY, // Sets: N, Z
    STA,
    STX,
    STY,

    // Register Transfers. All set N, Z
    TAX,
    TAY,
    TXA,
    TYA,
    TSX,
    TXS,

    // Stack Operations
    PHA, // Sets: N, Z
    PHP,
    PLA, // Sets: N, Z
    PLP, // Sets: ALL

    // Logical Operations. All set N, Z except BIT
    AND,
    EOR,
    ORA,
    BIT, // Sets: N, V, Z

    // Arithmetic. All set N, Z, C except ADC & SBC
    ADC, // Sets: N, V, Z, C
    SBC, // Sets: N, V, Z, C
    CMP,
    CPX,
    CPY,

    // Inc and Dec. All set N, Z
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,

    // Shifts. All set N, Z, C
    ASL,
    LSR,
    ROL,
    ROR,

    // Jumps and Calls. None set
    JMP,
    JSR,
    RTS,

    // Branches. None set
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,

    // Status Flag Changes
    CLC, // Clears C
    CLD, // Clears D
    CLI, // Clears I
    CLV, // Clears V
    SEC, // Sets C
    SED, // Sets D
    SEI, // Sets I

    // System Functions
    BRK, // Sets B
    NOP,
    RTI, // Sets All
}

#[derive(Debug, Clone)]
pub enum Flag {
    Zero,  // set if the result of the last operation was 0
    Carry, // set if the last operation caused an overflow from bit 7 or an
    // underflow from bit 0
    Interrupt, // set if the program has executed a 'set interrupt disable'
    // instruction
    Decimal,  // while set, cpu follows the rules of the Binary Coded Decimal
    Break,    // set when a break instruction is executed
    Overflow, // set if a math operation has overflowed the registers
    // eg. 64 + 64  = -128
    Negative, // set if the last operation set the 7th bit to a 1
}

impl InstructionType {
    pub fn flags(&self) -> Vec<Flag> {
        return match self {
            // Load/Store Operations
            InstructionType::LDA | InstructionType::LDX | InstructionType::LDY => {
                vec![Flag::Negative, Flag::Zero]
            }
            InstructionType::STA | InstructionType::STX | InstructionType::STY => vec![],

            // Register Transfers. All set N, Z
            InstructionType::TAX
            | InstructionType::TAY
            | InstructionType::TXA
            | InstructionType::TYA
            | InstructionType::TSX
            | InstructionType::TXS => vec![Flag::Negative, Flag::Zero],

            // Stack Operations
            InstructionType::PHA | InstructionType::PLA => vec![Flag::Negative, Flag::Zero],
            InstructionType::PHP => vec![],
            InstructionType::PLP => vec![
                Flag::Zero,
                Flag::Carry,
                Flag::Interrupt,
                Flag::Decimal,
                Flag::Break,
                Flag::Overflow,
                Flag::Negative,
            ],

            // Logical Operations
            InstructionType::AND | InstructionType::EOR | InstructionType::ORA => {
                vec![Flag::Negative, Flag::Zero]
            }
            InstructionType::BIT => vec![Flag::Negative, Flag::Overflow, Flag::Zero],

            // Arithmetic
            InstructionType::ADC | InstructionType::SBC => {
                vec![Flag::Negative, Flag::Overflow, Flag::Zero, Flag::Carry]
            }
            InstructionType::CMP | InstructionType::CPX | InstructionType::CPY => {
                vec![Flag::Negative, Flag::Zero, Flag::Carry]
            }

            // Inc and Dec
            InstructionType::INC
            | InstructionType::INX
            | InstructionType::INY
            | InstructionType::DEC
            | InstructionType::DEX
            | InstructionType::DEY => vec![Flag::Negative, Flag::Zero],

            // Shifts
            InstructionType::ASL
            | InstructionType::LSR
            | InstructionType::ROL
            | InstructionType::ROR => vec![Flag::Negative, Flag::Zero, Flag::Carry],

            // Jumps and Calls
            InstructionType::JMP | InstructionType::JSR | InstructionType::RTS => vec![],

            // Branches
            InstructionType::BCC
            | InstructionType::BCS
            | InstructionType::BEQ
            | InstructionType::BMI
            | InstructionType::BNE
            | InstructionType::BPL
            | InstructionType::BVC
            | InstructionType::BVS => vec![],

            // Status Flag Changes
            InstructionType::CLC | InstructionType::SEC => vec![Flag::Carry],
            InstructionType::CLD | InstructionType::SED => vec![Flag::Decimal],
            InstructionType::CLI | InstructionType::SEI => vec![Flag::Interrupt],
            InstructionType::CLV => vec![Flag::Overflow],

            // System Functions
            InstructionType::BRK => vec![Flag::Break],
            InstructionType::NOP => vec![],
            InstructionType::RTI => vec![
                Flag::Zero,
                Flag::Carry,
                Flag::Interrupt,
                Flag::Decimal,
                Flag::Break,
                Flag::Overflow,
                Flag::Negative,
            ],
        };
    }
}

impl FromStr for InstructionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LDA" => Ok(InstructionType::LDA),
            "LDX" => Ok(InstructionType::LDX),
            "LDY" => Ok(InstructionType::LDY),
            "STA" => Ok(InstructionType::STA),
            "STX" => Ok(InstructionType::STX),
            "STY" => Ok(InstructionType::STY),
            "TAX" => Ok(InstructionType::TAX),
            "TAY" => Ok(InstructionType::TAY),
            "TXA" => Ok(InstructionType::TXA),
            "TYA" => Ok(InstructionType::TYA),
            "TSX" => Ok(InstructionType::TSX),
            "TXS" => Ok(InstructionType::TXS),
            "PHA" => Ok(InstructionType::PHA),
            "PHP" => Ok(InstructionType::PHP),
            "PLA" => Ok(InstructionType::PLA),
            "PLP" => Ok(InstructionType::PLP),
            "AND" => Ok(InstructionType::AND),
            "EOR" => Ok(InstructionType::EOR),
            "ORA" => Ok(InstructionType::ORA),
            "BIT" => Ok(InstructionType::BIT),
            "ADC" => Ok(InstructionType::ADC),
            "SBC" => Ok(InstructionType::SBC),
            "CMP" => Ok(InstructionType::CMP),
            "CPX" => Ok(InstructionType::CPX),
            "CPY" => Ok(InstructionType::CPY),
            "INC" => Ok(InstructionType::INC),
            "INX" => Ok(InstructionType::INX),
            "INY" => Ok(InstructionType::INY),
            "DEC" => Ok(InstructionType::DEC),
            "DEX" => Ok(InstructionType::DEX),
            "DEY" => Ok(InstructionType::DEY),
            "ASL" => Ok(InstructionType::ASL),
            "LSR" => Ok(InstructionType::LSR),
            "ROL" => Ok(InstructionType::ROL),
            "ROR" => Ok(InstructionType::ROR),
            "JMP" => Ok(InstructionType::JMP),
            "JSR" => Ok(InstructionType::JSR),
            "RTS" => Ok(InstructionType::RTS),
            "BCC" => Ok(InstructionType::BCC),
            "BCS" => Ok(InstructionType::BCS),
            "BEQ" => Ok(InstructionType::BEQ),
            "BMI" => Ok(InstructionType::BMI),
            "BNE" => Ok(InstructionType::BNE),
            "BPL" => Ok(InstructionType::BPL),
            "BVC" => Ok(InstructionType::BVC),
            "BVS" => Ok(InstructionType::BVS),
            "CLC" => Ok(InstructionType::CLC),
            "CLD" => Ok(InstructionType::CLD),
            "CLI" => Ok(InstructionType::CLI),
            "CLV" => Ok(InstructionType::CLV),
            "SEC" => Ok(InstructionType::SEC),
            "SED" => Ok(InstructionType::SED),
            "SEI" => Ok(InstructionType::SEI),
            "BRK" => Ok(InstructionType::BRK),
            "NOP" => Ok(InstructionType::NOP),
            "RTI" => Ok(InstructionType::RTI),
            _ => Err(format!("Unknown instruction: {}", s)),
        }
    }
}
