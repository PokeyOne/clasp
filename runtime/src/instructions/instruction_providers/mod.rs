use super::*;
use clasp_common::instruction_constants::{base_code_from_instruction_type, INSTRUCTIONS};

mod console;
mod general;
mod jumping;
mod math;
mod movement;

type InstructionProvider = fn(&mut Memory, &mut Memory, &mut MemoryLocation);

static INSTRUCTION_FUNCTIONS: phf::Map<u64, InstructionProvider> = phf_map! {
    0x0u64 => general::nop_provider,
    0x1u64 => movement::mov_provider,
    0x2u64 => math::add_provider,
    0x3u64 => math::sub_provider,
    0x4u64 => math::mul_provider,
    0x5u64 => math::div_provider,
    0x6u64 => math::pow_provider,
    0x7u64 => general::end_provider,
    0x8u64 => movement::movr_provider,
    0x9u64 => console::outr_provider,
    0xAu64 => console::outr_provider,
    0xBu64 => jumping::jmp_provider,
    0xCu64 => jumping::jmp_provider
};

pub fn perform(
    inst: u64,
    memory: &mut Memory,
    program_memory: &mut Memory,
    program_counter: &mut MemoryLocation
) {
    let base_inst = INSTRUCTIONS.get(&inst).unwrap();
    let base_inst = base_code_from_instruction_type(base_inst);

    let method: &InstructionProvider = INSTRUCTION_FUNCTIONS
        .get(&base_inst)
        .expect("Unimplemented instruction");
    method(memory, program_memory, program_counter);
}
