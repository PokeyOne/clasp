use crate::memory::constants::*;
use crate::memory::types::{MemoryLocation, Word};
use crate::memory::Memory;
use crate::memory::types::Result as MemoryResult;
use crate::memory::types::Status as MemoryStatus;
use phf::phf_map;

// Rexport perform function
pub use instruction_providers::perform;

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


mod instruction_providers {
    use super::*;

    type InstructionProvider = fn(&mut Memory, &mut MemoryLocation);

    static INSTRUCTION_FUNCTIONS: phf::Map<u64, InstructionProvider> = phf_map! {
        0u64 => nop_provider,
        1u64 => mov_provider,
        2u64 => add_provider,
        3u64 => sub_provider,
        4u64 => mul_provider,
        5u64 => div_provider,
        6u64 => pow_provider,
        7u64 => end_provider
    };

    pub fn perform(inst: u64, memory: &mut Memory, program_counter: &mut MemoryLocation) {
        println!("Running instruction {:?}", INSTRUCTIONS.get(&inst));
        let method: &InstructionProvider = INSTRUCTION_FUNCTIONS
            .get(&inst)
            .expect("Unimplemented instruction");
        method(memory, program_counter);
    }

    fn nop_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc += WORD_SIZE as u64;
        return;
    }

    // TODO: Move raw instruction for moving a litteral
    fn mov_provider(mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc += WORD_SIZE as u64;

        let source_location: Word = match (*mem).read((*pc).clone()) {
            MemoryResult::Ok(val) => val,
            MemoryResult::Err(t) => panic!("Could not read move instruction source location word due to memory error type: {:?}", t)
        };
        *pc += WORD_SIZE as u64;
        let source_value: Word = match (*mem).read(source_location) {
            MemoryResult::Ok(val) => val,
            MemoryResult::Err(t) => panic!("Could not read move instruction source value word due to memory error type: {:?}", t)
        };

        let destination_location: Word = match (*mem).read((*pc).clone()) {
            MemoryResult::Ok(val) => val,
            MemoryResult::Err(t) => panic!("Could not read move instruction source location word due to memory error type: {:?}", t)
        };
        *pc += WORD_SIZE as u64;

        println!(
            "About to move {:016X} from {:016X} to {:016X}",
            source_value,
            source_location,
            destination_location
        );
        match (*mem).write(destination_location, source_value) {
            MemoryStatus::Ok => {},
            MemoryStatus::Err(t) => panic!("Moving data to memory threw error: {:?}", t)
        };
        return;
    }

    fn add_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc += WORD_SIZE as u64;
        return;
    }

    fn sub_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc += WORD_SIZE as u64;
        return;
    }

    fn mul_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc += WORD_SIZE as u64;
        return;
    }

    fn div_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc += WORD_SIZE as u64;
        return;
    }

    fn pow_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc += WORD_SIZE as u64;
        return;
    }

    fn end_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        *pc = 0xFFFF_FFFF_FFFF_FFFFu64;
        return;
    }
}
