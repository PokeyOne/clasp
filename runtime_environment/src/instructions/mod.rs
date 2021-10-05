use crate::memory::constants::*;
use crate::memory::types::Result as MemoryResult;
use crate::memory::types::Status as MemoryStatus;
use crate::memory::types::{MemoryLocation, Word};
use crate::memory::Memory;
use phf::phf_map;

// Rexport perform function
mod instruction_providers;
pub use instruction_providers::perform;

// TODO: These have been copied to clasp_common::instruction_constants
#[derive(Debug)]
pub enum InstructionType {
    NOP,
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    POW,
    END
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
    0x0000_0000_0000_0007u64 => END
};
