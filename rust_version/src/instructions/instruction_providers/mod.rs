use super::*;

mod general;
mod math;
mod movement;

type InstructionProvider = fn(&mut Memory, &mut MemoryLocation);

static INSTRUCTION_FUNCTIONS: phf::Map<u64, InstructionProvider> = phf_map! {
    0u64 => general::nop_provider,
    1u64 => movement::mov_provider,
    2u64 => math::add_provider,
    3u64 => math::sub_provider,
    4u64 => math::mul_provider,
    5u64 => math::div_provider,
    6u64 => math::pow_provider,
    7u64 => general::end_provider
};

pub fn perform(inst: u64, memory: &mut Memory, program_counter: &mut MemoryLocation) {
    println!("Running instruction {:?}", INSTRUCTIONS.get(&inst));
    let method: &InstructionProvider = INSTRUCTION_FUNCTIONS
        .get(&inst)
        .expect("Unimplemented instruction");
    method(memory, program_counter);
}
