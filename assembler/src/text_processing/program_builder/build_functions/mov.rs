#[cfg(test)]
mod tests;

use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::{MOV_CODE, MOVR_CODE};
use clasp_common::data_constants::{register_locations, get_register_address};
use crate::text_processing::program_builder::{Instruction, InstructionBuildError, InstructionBuildError::*, Operand};

/// Builds a MOV instruction.
pub fn mov(instruction: &Instruction) -> Result<Vec<u8>, InstructionBuildError> {
    // Create vec with 24 bytes reserved because the instruction is
    // 3 words long, and each word is 64 bits or 8 bytes long.
    let mut result: Vec<u8> = Vec::new();
    result.reserve(24);

    // Determine the instruction code and add each byte to the vec.
    for d in inst_code(instruction).to_bytes() {
        result.push(d);
    }

    // Add the first operand bytes to the vec
    for d in instruction.get_operands()[0].to_be_bytes() {
        result.push(d);
    }

    // Add the second operand bytes to the vec
    match &instruction.get_operands()[1] {
        Operand::Immediate(_) => return Err(ExpectedAddressableOperand),
        Operand::Address(addr) => {
            for d in addr.to_be_bytes() {
                result.push(d);
            }
        }
    }

    Ok(result)
}

fn inst_code(instruction: &Instruction) -> u64 {
    match instruction.get_operands()[0] {
        Operand::Immediate(_) => MOVR_CODE,
        Operand::Address(_) => MOV_CODE,
    }
}
