use memory::types::{Word};
use phf::phf_map;

pub enum InstructionType {
    NOP,
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    POW
}

use InstructionType::*;

pub static INSTRUCTIONS: phf::Map<static Word, InstructionType> = phf_map! {
    0x0000_0000_0000_0000 => NOP,
    0x0000_0000_0000_0001 => MOV,
    0x0000_0000_0000_0002 => ADD,
    0x0000_0000_0000_0003 => SUB,
    0x0000_0000_0000_0004 => MUL,
    0x0000_0000_0000_0005 => DIV,
    0x0000_0000_0000_0006 => POW
};

pub static INSTRUCTION_CODES: phf::Map<InstructionType, Word> = phf_map! {
    NOP => 0x0000_0000_0000_0000,
    MOV => 0x0000_0000_0000_0001,
    ADD => 0x0000_0000_0000_0002,
    SUB => 0x0000_0000_0000_0003,
    MUL => 0x0000_0000_0000_0004,
    DIV => 0x0000_0000_0000_0005,
    POW => 0x0000_0000_0000_0006
}
