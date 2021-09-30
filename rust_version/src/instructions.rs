use memory::types::{Word};
use phf::phf_map;

pub enum InstructionType {
    NOP,
    MOV
}

use InstructionType::*;

pub static INSTRUCTIONS: phf::Map<static Word, InstructionType> = phf_map! {
    0x0000_0000_0000_0000 => NOP,
    0x0000_0000_0000_0001 => MOV
};

pub static INSTRUCTION_CODES: phf::Map<InstructionType, Word> = phf_map! {
    NOP => 0x0000_0000_0000_0000,
    MOV => 0x0000_0000_0000_0001
}
