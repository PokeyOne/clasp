use crate::data_block;
use crate::text_processing::program_builder::{InstructionKind, OperandType};
use super::*;

#[test]
fn mov_address_to_address() -> Result<(), InstructionBuildError> {
    let instruction = Instruction::new(
        InstructionKind::Mov,
        vec![
            Operand::Address(0x00),
            Operand::Address(0x08),
        ],
    );
    let result = instruction.build()?;

    assert_eq!(result, data_block![MOV_CODE, 0x00, 0x08]);

    Ok(())
}

#[test]
fn mov_address_to_register() -> Result<(), InstructionBuildError> {
    let instruction = Instruction::new(
        InstructionKind::Mov,
        vec![
            Operand::Address(0x00),
            Operand::Register("ga".to_string()),
        ],
    );
    let result = instruction.build()?;

    assert_eq!(result, data_block![MOV_CODE, 0x00, register_locations::GA_LOC]);

    Ok(())
}