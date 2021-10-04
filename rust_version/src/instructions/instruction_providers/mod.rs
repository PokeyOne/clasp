use super::*;

mod general;
mod math;
mod movement;

type InstructionProvider = fn(&mut Memory, &mut MemoryLocation);

static INSTRUCTION_FUNCTIONS: phf::Map<u64, InstructionProvider> = phf_map! {
    0u64 => general::nop_provider,
    1u64 => mov_provider,
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

// TODO: Move raw instruction for moving a litteral
fn mov_provider(mem: &mut Memory, pc: &mut MemoryLocation) {
    *pc += WORD_SIZE as u64;

    let source_location: Word = match (*mem).read((*pc).clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;
    let source_value: Word = match (*mem).read(source_location) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source value word due to memory error type: {:?}",
            t
        )
    };

    let destination_location: Word = match (*mem).read((*pc).clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;

    println!(
        "About to move {:016X} from {:016X} to {:016X}",
        source_value, source_location, destination_location
    );
    match (*mem).write(destination_location, source_value) {
        MemoryStatus::Ok => {}
        MemoryStatus::Err(t) => panic!("Moving data to memory threw error: {:?}", t)
    };
    return;
}
