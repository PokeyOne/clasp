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

// TODO: Convert these to UpperCamelCase
#[derive(Debug)]
pub enum InstructionType {
    NOP,
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    END,
    MOVR, // move raw
    OUTR_ADDR, // out raw, address
    OUTR_LIT // out raw, literal
}
use InstructionType::*;

pub static INSTRUCTIONS: phf::Map<u64, InstructionType> = phf_map! {
    0x0000_0000_0000_0000u64 => NOP,
    0x0000_0000_0000_0001u64 => MOV,
    0x0000_0000_0000_0002u64 => ADD,
    0x1000_0000_0000_0002u64 => ADD,
    0x2000_0000_0000_0002u64 => ADD,
    0x3000_0000_0000_0002u64 => ADD,
    0x0000_0000_0000_0003u64 => SUB,
    0x1000_0000_0000_0003u64 => SUB,
    0x2000_0000_0000_0003u64 => SUB,
    0x3000_0000_0000_0003u64 => SUB,
    0x0000_0000_0000_0004u64 => MUL,
    0x1000_0000_0000_0004u64 => MUL,
    0x2000_0000_0000_0004u64 => MUL,
    0x3000_0000_0000_0004u64 => MUL,
    0x0000_0000_0000_0005u64 => DIV,
    0x1000_0000_0000_0005u64 => DIV,
    0x2000_0000_0000_0005u64 => DIV,
    0x3000_0000_0000_0005u64 => DIV,
    0x0000_0000_0000_0006u64 => POW,
    0x1000_0000_0000_0006u64 => POW,
    0x2000_0000_0000_0006u64 => POW,
    0x3000_0000_0000_0006u64 => POW,
    0x0000_0000_0000_0007u64 => END,
    0x0000_0000_0000_0008u64 => MOVR,
    0x0000_0000_0000_0009u64 => OUTR_ADDR,
    0x0000_0000_0000_000Au64 => OUTR_LIT
};
