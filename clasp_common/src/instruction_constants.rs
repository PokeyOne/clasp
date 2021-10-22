use super::data_types::Word;
use phf::phf_map;

/// All the raw 64-bit instruction codes. In there own module so that you
/// can short-hand import all of them at once. They are also re-exported by
/// this modules superior
pub mod instruction_codes {
    use super::Word;

    pub const MATH_MOD_CC: Word = 0x0000_0000_0000_0000u64; // 2 constants
    pub const MATH_MOD_CA: Word = 0x1000_0000_0000_0000u64; // address, constant
    pub const MATH_MOD_AC: Word = 0x2000_0000_0000_0000u64; // constant, address
    pub const MATH_MOD_AA: Word = 0x3000_0000_0000_0000u64; // 2 addresses

    pub const NOP_CODE: Word = 0x0000_0000_0000_0000u64;
    pub const MOV_CODE: Word = 0x0000_0000_0000_0001u64;
    pub const ADD_CODE: Word = 0x0000_0000_0000_0002u64;
    pub const ADD_CODE_CC: Word = ADD_CODE + MATH_MOD_CC;
    pub const ADD_CODE_CA: Word = ADD_CODE + MATH_MOD_CA;
    pub const ADD_CODE_AC: Word = ADD_CODE + MATH_MOD_AC;
    pub const ADD_CODE_AA: Word = ADD_CODE + MATH_MOD_AA;
    pub const SUB_CODE: Word = 0x0000_0000_0000_0003u64;
    pub const SUB_CODE_CC: Word = SUB_CODE + MATH_MOD_CC;
    pub const SUB_CODE_CA: Word = SUB_CODE + MATH_MOD_CA;
    pub const SUB_CODE_AC: Word = SUB_CODE + MATH_MOD_AC;
    pub const SUB_CODE_AA: Word = SUB_CODE + MATH_MOD_AA;
    pub const MUL_CODE: Word = 0x0000_0000_0000_0004u64;
    pub const MUL_CODE_CC: Word = MUL_CODE + MATH_MOD_CC;
    pub const MUL_CODE_CA: Word = MUL_CODE + MATH_MOD_CA;
    pub const MUL_CODE_AC: Word = MUL_CODE + MATH_MOD_AC;
    pub const MUL_CODE_AA: Word = MUL_CODE + MATH_MOD_AA;
    pub const DIV_CODE: Word = 0x0000_0000_0000_0005u64;
    pub const DIV_CODE_CC: Word = DIV_CODE + MATH_MOD_CC;
    pub const DIV_CODE_CA: Word = DIV_CODE + MATH_MOD_CA;
    pub const DIV_CODE_AC: Word = DIV_CODE + MATH_MOD_AC;
    pub const DIV_CODE_AA: Word = DIV_CODE + MATH_MOD_AA;
    pub const POW_CODE: Word = 0x0000_0000_0000_0006u64;
    pub const POW_CODE_CC: Word = POW_CODE + MATH_MOD_CC;
    pub const POW_CODE_CA: Word = POW_CODE + MATH_MOD_CA;
    pub const POW_CODE_AC: Word = POW_CODE + MATH_MOD_AC;
    pub const POW_CODE_AA: Word = POW_CODE + MATH_MOD_AA;
    pub const END_CODE: Word = 0x0000_0000_0000_0007u64;
    pub const MOVR_CODE: Word = 0x0000_0000_0000_0008u64;
    pub const OUTR_ADDR_CODE: Word = 0x0000_0000_0000_0009u64;
    pub const OUTR_LIT_CODE: Word = 0x0000_0000_0000_000Au64;
}
pub use instruction_codes::*;

#[derive(Debug)]
pub enum InstructionType {
    Nop,
    Mov,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    End,
    Movr,     // move raw
    OutrAddr, // out raw, address
    OutrLit   // out raw, literal
}
use InstructionType::*;

pub static INSTRUCTIONS: phf::Map<u64, InstructionType> = phf_map! {
    0x0000_0000_0000_0000u64 => Nop,
    0x0000_0000_0000_0001u64 => Mov,
    0x0000_0000_0000_0002u64 => Add,
    0x1000_0000_0000_0002u64 => Add,
    0x2000_0000_0000_0002u64 => Add,
    0x3000_0000_0000_0002u64 => Add,
    0x0000_0000_0000_0003u64 => Sub,
    0x1000_0000_0000_0003u64 => Sub,
    0x2000_0000_0000_0003u64 => Sub,
    0x3000_0000_0000_0003u64 => Sub,
    0x0000_0000_0000_0004u64 => Mul,
    0x1000_0000_0000_0004u64 => Mul,
    0x2000_0000_0000_0004u64 => Mul,
    0x3000_0000_0000_0004u64 => Mul,
    0x0000_0000_0000_0005u64 => Div,
    0x1000_0000_0000_0005u64 => Div,
    0x2000_0000_0000_0005u64 => Div,
    0x3000_0000_0000_0005u64 => Div,
    0x0000_0000_0000_0006u64 => Pow,
    0x1000_0000_0000_0006u64 => Pow,
    0x2000_0000_0000_0006u64 => Pow,
    0x3000_0000_0000_0006u64 => Pow,
    0x0000_0000_0000_0007u64 => End,
    0x0000_0000_0000_0008u64 => Movr,
    0x0000_0000_0000_0009u64 => OutrAddr,
    0x0000_0000_0000_000Au64 => OutrLit
};

pub fn base_code_from_instruction_type(instruction_type: &InstructionType) -> u64 {
    match instruction_type {
        Nop => 0x0000_0000_0000_0000u64,
        Mov => 0x0000_0000_0000_0001u64,
        Add => 0x0000_0000_0000_0002u64,
        Sub => 0x0000_0000_0000_0003u64,
        Mul => 0x0000_0000_0000_0004u64,
        Div => 0x0000_0000_0000_0005u64,
        Pow => 0x0000_0000_0000_0006u64,
        End => 0x0000_0000_0000_0007u64,
        Movr => 0x0000_0000_0000_0008u64,
        OutrAddr => 0x0000_0000_0000_0009u64,
        OutrLit => 0x0000_0000_0000_000Au64
    }
}
