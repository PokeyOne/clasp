use super::data_types::Word;
use phf::phf_map;

/// All the raw 64-bit instruction codes. In there own module so that you
/// can short-hand import all of them at once. They are also re-exported by
/// this modules superior
pub mod instruction_codes {
    use super::Word;

    pub const NOP_CODE: Word = 0x0000_0000_0000_0000u64;
    pub const MOV_CODE: Word = 0x0000_0000_0000_0001u64;
    pub const ADD_CODE: Word = 0x0000_0000_0000_0002u64;
    pub const SUB_CODE: Word = 0x0000_0000_0000_0003u64;
    pub const MUL_CODE: Word = 0x0000_0000_0000_0004u64;
    pub const DIV_CODE: Word = 0x0000_0000_0000_0005u64;
    pub const POW_CODE: Word = 0x0000_0000_0000_0006u64;
    pub const END_CODE: Word = 0x0000_0000_0000_0007u64;
    pub const MOVR_CODE: Word = 0x0000_0000_0000_0008u64;
}
pub use instruction_codes::*;

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
    MOVR // move raw
}
use InstructionType::*;

pub static INSTRUCTIONS: phf::Map<u64, InstructionType> = phf_map! {
    0x0000_0000_0000_0000u64 => NOP,
    0x0000_0000_0000_0001u64 => MOV,
    0x0000_0000_0000_0002u64 => ADD,
    0x0000_0000_0000_0003u64 => SUB,
    0x0000_0000_0000_0004u64 => MUL,
    0x0000_0000_0000_0005u64 => DIV,
    0x0000_0000_0000_0006u64 => POW,
    0x0000_0000_0000_0007u64 => END,
    0x0000_0000_0000_0008u64 => MOVR
};
