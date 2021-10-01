use memory::types::{Word, MemoryLocation};
use memory::constants::*;
use memory::Memory;
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

mod instruction_providers {
    static INSTRUCTION_FUNCTIONS: phf::Map<InstructionType, fn(&mut Memory, &mut MemoryLocation)> phf_map! {
        NOP => nop_provider,
        MOV => mov_provider,
        ADD => add_provider,
        SUB => sub_provider,
        MUL => mul_provider,
        DIV => div_provider,
        POW => pow_provider
    }

    pub fn perform(inst: InstructionType, memory: &mut Memory, program_counter: &mut MemoryLocation) {
        INSTRUCTION_FUNCTIONS.get(inst)(memory, program_counter);
    }

    fn nop_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
        pc += WORD_SIZE;
        return;
    }

    fn mov_provider(_mem: &mut Memory, _pc: &mut MemoryLocation) {
        pc += WORD_SIZE;
        return;
    }

    fn add_provider(_mem: &mut Memory, _pc: &mut MemoryLocation) {
        pc += WORD_SIZE;
        return;
    }

    fn sub_provider(_mem: &mut Memory, _pc: &mut MemoryLocation) {
        pc += WORD_SIZE;
        return;
    }

    fn mul_provider(_mem: &mut Memory, _pc: &mut MemoryLocation) {
        pc += WORD_SIZE;
        return;
    }

    fn div_provider(_mem: &mut Memory, _pc: &mut MemoryLocation) {
        pc += WORD_SIZE;
        return;
    }

    fn pow_provider(_mem: &mut Memory, _pc: &mut MemoryLocation) {
        pc += WORD_SIZE;
        return;
    }
}
