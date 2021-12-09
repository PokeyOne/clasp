#[cfg(test)]
mod tests;

use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::{MOV_CODE, MOVR_CODE};
use clasp_common::data_constants::register_locations;
use crate::text_processing::program_builder::{Instruction, InstructionBuildError, InstructionBuildError::*, Operand};

/// Builds a MOV instruction.
pub fn mov(instruction: &Instruction) -> Result<Vec<u8>, InstructionBuildError> {
    let mut result: Vec<u8> = Vec::new();
    result.reserve(12);
    for d in inst_code(instruction).to_bytes() {
        result.push(d);
    }

    Err(NotImplemented)
}

fn inst_code(instruction: &Instruction) -> u64 {
    match instruction.get_operands()[0] {
        Operand::Immediate(_) => MOVR_CODE,
        _ => MOV_CODE,
    }
}
