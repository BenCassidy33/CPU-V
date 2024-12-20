use std::str::FromStr;

use egui::TextBuffer;

#[derive(Debug)]
pub struct Program {
    pub data: Vec<Variable>,
    pub labels: Option<Vec<Label>>,
    pub env: Option<Env>,
}

impl Program {
    pub fn new() -> Self {
        return Program {
            data: Vec::new(),
            labels: None,
            env: None,
        };
    }
}

#[derive(Debug)]
pub struct Data {}
#[derive(Debug)]
pub struct Env {}

//#[derive(Debug, Clone)]
//pub enum Section {
//    Program(Option<Vec<Label>>),
//    Data(Option<Variable>),
//    Env,

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum DataType {
    Byte(DataSizes::Byte),
    SByte(DataSizes::SByte),
    Word(DataSizes::Word),
    SWord(DataSizes::SWord),
    DWord(DataSizes::DWord),
    SDWord(DataSizes::SDWord),
    QWord(DataSizes::QWord),
    TByte(DataSizes::TByte),
    Real4(DataSizes::Real4),
    Real8(DataSizes::Real8),
    Str4(DataSizes::Str4),
    Str8(DataSizes::Str8),
    Str16(DataSizes::Str16),
    Str32(DataSizes::Str32),
    Str64(DataSizes::Str64),
    Str128(DataSizes::Str128),
}

#[derive(Debug, Clone)]
pub struct DataTypeInfo {}

impl FromStr for DataType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "byte" => Ok(DataType::Byte(0)),
            "sbyte" => Ok(DataType::SByte(0)),
            "word" => Ok(DataType::Word(0)),
            "sword" => Ok(DataType::SWord(0)),
            "dword" => Ok(DataType::DWord(0)),
            "sdword" => Ok(DataType::SDWord(0)),
            "qword" => Ok(DataType::QWord(0)),
            "tbyte" => Ok(DataType::TByte([0; 10])),
            "real4" => Ok(DataType::Real4(0.0)),
            "real8" => Ok(DataType::Real8(0.0)),
            "str4" => Ok(DataType::Str4([' '; 4])),
            "str8" => Ok(DataType::Str8([' '; 8])),
            "str16" => Ok(DataType::Str16([' '; 16])),
            "str32" => Ok(DataType::Str32([' '; 32])),
            "str64" => Ok(DataType::Str64([' '; 64])),
            "str128" => Ok(DataType::Str128([' '; 128])),
            _ => Err(format!("Invalid DataType string: {}", input)),
        }
    }
}

#[derive(Debug)]
pub enum SectionError {
    InvalidSectionType,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub ty: DataType,
    pub value: String, // TODO: Make sure that type matches value
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub ty: InstructionType,
    pub val: InstructionValue,
    pub flags: Vec<Flag>,
}

#[derive(Debug, Clone)]
enum InstructionValue {
    SingleValue(String),
    MultipleValue((String, String)),
}

#[derive(Debug, Clone, PartialEq)]
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
    JEQ,
    JNE,
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

    // Custom
    CAL, // Creates a funtion call to the os, Sets Interrupt
    MOV, // Moves one value into another eg. mov from, to, Sets Interrupt
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
            | InstructionType::BVS
            | InstructionType::JEQ
            | InstructionType::JNE => vec![],

            // Status Flag Changes
            InstructionType::CLC | InstructionType::SEC => vec![Flag::Carry],
            InstructionType::CLD | InstructionType::SED => vec![Flag::Decimal],
            InstructionType::CLI
            | InstructionType::SEI
            | InstructionType::CAL
            | InstructionType::MOV => {
                vec![Flag::Interrupt]
            }
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

    pub fn has_multiple_values(&self) -> bool {
        return match self {
            Self::JMP => true,
            _ => false,
        };
    }

    pub fn is_valueless(&self) -> bool {
        return match self {
            Self::TAX
            | Self::TAY
            | Self::TXA
            | Self::TYA
            | Self::TSX
            | Self::TXS
            | Self::PHA
            | Self::PHP
            | Self::PLA
            | Self::PLP => true,
            _ => false,
        };
    }
}

impl FromStr for InstructionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lda" => Ok(InstructionType::LDA),
            "ldx" => Ok(InstructionType::LDX),
            "ldy" => Ok(InstructionType::LDY),
            "sta" => Ok(InstructionType::STA),
            "stx" => Ok(InstructionType::STX),
            "sty" => Ok(InstructionType::STY),
            "tax" => Ok(InstructionType::TAX),
            "tay" => Ok(InstructionType::TAY),
            "txa" => Ok(InstructionType::TXA),
            "tya" => Ok(InstructionType::TYA),
            "tsx" => Ok(InstructionType::TSX),
            "txs" => Ok(InstructionType::TXS),
            "pha" => Ok(InstructionType::PHA),
            "php" => Ok(InstructionType::PHP),
            "pla" => Ok(InstructionType::PLA),
            "plp" => Ok(InstructionType::PLP),
            "and" => Ok(InstructionType::AND),
            "eor" => Ok(InstructionType::EOR),
            "ora" => Ok(InstructionType::ORA),
            "bit" => Ok(InstructionType::BIT),
            "adc" => Ok(InstructionType::ADC),
            "sbc" => Ok(InstructionType::SBC),
            "cmp" => Ok(InstructionType::CMP),
            "cpx" => Ok(InstructionType::CPX),
            "cpy" => Ok(InstructionType::CPY),
            "inc" => Ok(InstructionType::INC),
            "inx" => Ok(InstructionType::INX),
            "iny" => Ok(InstructionType::INY),
            "dec" => Ok(InstructionType::DEC),
            "dex" => Ok(InstructionType::DEX),
            "dey" => Ok(InstructionType::DEY),
            "asl" => Ok(InstructionType::ASL),
            "lsr" => Ok(InstructionType::LSR),
            "rol" => Ok(InstructionType::ROL),
            "ror" => Ok(InstructionType::ROR),
            "jmp" => Ok(InstructionType::JMP),
            "jsr" => Ok(InstructionType::JSR),
            "rts" => Ok(InstructionType::RTS),
            "bcc" => Ok(InstructionType::BCC),
            "bcs" => Ok(InstructionType::BCS),
            "beq" => Ok(InstructionType::BEQ),
            "bmi" => Ok(InstructionType::BMI),
            "bne" => Ok(InstructionType::BNE),
            "bpl" => Ok(InstructionType::BPL),
            "bvc" => Ok(InstructionType::BVC),
            "bvs" => Ok(InstructionType::BVS),
            "clc" => Ok(InstructionType::CLC),
            "cld" => Ok(InstructionType::CLD),
            "cli" => Ok(InstructionType::CLI),
            "clv" => Ok(InstructionType::CLV),
            "sec" => Ok(InstructionType::SEC),
            "sed" => Ok(InstructionType::SED),
            "sei" => Ok(InstructionType::SEI),
            "brk" => Ok(InstructionType::BRK),
            "nop" => Ok(InstructionType::NOP),
            "rti" => Ok(InstructionType::RTI),
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
            flags: ty.flags(),
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

#[derive(Default)]
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
