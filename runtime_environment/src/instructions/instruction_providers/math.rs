use super::*;
use clasp_common::instruction_constants::{base_code_from_instruction_type as bcfit, InstructionType, instruction_codes::*};
use std::convert::TryInto;

fn get_arguments(memory: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation, inst_type: InstructionType) -> (u64, u64, MemoryLocation) {
    let op_code = match program_memory.read(pc.clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!("{:?}", t)
    };
    *pc += WORD_SIZE as u64;

    let mut is_constant: [bool; 2] = match op_code - bcfit(&inst_type) {
        MATH_MOD_CC => [true, true],
        MATH_MOD_CA => [true, false],
        MATH_MOD_AC => [false, true],
        MATH_MOD_AA => [false, false],
        _ => panic!("Unknown math mod code for operator {} and type (bcfit: {}) {:?}", op_code, bcfit(&inst_type), inst_type)
    };

    let mut vals: [u64; 2] = [0u64, 0u64];
    for i in [0, 1] {
        // This is the actual value if this is a constants, but it is the
        // address if we are not dealing with an constant.
        let initial_value = match program_memory.read(pc.clone()) {
            MemoryResult::Ok(val) => val,
            MemoryResult::Err(t) => panic!("{:?}", t)
        };
        *pc += WORD_SIZE as u64;

        vals[i] = match is_constant[i] {
            true => initial_value,
            false => match memory.read(initial_value) {
                MemoryResult::Ok(val) => val,
                MemoryResult::Err(t) => panic!("{:?}", t)
            }
        };
    }

    let destination = match program_memory.read(pc.clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!("{:?}", t)
    };
    *pc += WORD_SIZE as u64;

    return (vals[0], vals[1], destination);
}

pub fn add_provider(memory: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    let (alpha, beta, dest) = get_arguments(memory, program_memory, pc, InstructionType::Add);

    let result = alpha + beta;

    memory.write(dest, result);
}

pub fn sub_provider(memory: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    let (alpha, beta, dest) = get_arguments(memory, program_memory, pc, InstructionType::Sub);

    let result = alpha - beta;

    memory.write(dest, result);
}

pub fn mul_provider(memory: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    let (alpha, beta, dest) = get_arguments(memory, program_memory, pc, InstructionType::Mul);

    let result = alpha * beta;

    memory.write(dest, result);
}

pub fn div_provider(memory: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    let (alpha, beta, dest) = get_arguments(memory, program_memory, pc, InstructionType::Div);

    let result = alpha / beta;

    memory.write(dest, result);
}

pub fn pow_provider(memory: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    let (base, pow, dest) = get_arguments(memory, program_memory, pc, InstructionType::Pow);

    let result = if base == 2 {
        2 << pow-1
    } else {
        base.pow(pow.try_into().unwrap())
    };

    memory.write(dest, result);
}
