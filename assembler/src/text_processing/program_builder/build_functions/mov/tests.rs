use crate::data_block;
use crate::text_processing::program_builder::{InstructionKind, OperandType};
use super::*;

#[test]
fn mov_address_to_address() -> Result<(), InstructionBuildError> {
    let instruction = Instruction::new(
        InstructionKind::Mov,
        vec![
            Operand::Address(0x12),
            Operand::Address(0x08),
        ],
    );
    let result = instruction.build()?;

    assert_eq!(result, data_block![MOV_CODE, 0x12, 0x08]);

    Ok(())
}

#[test]
fn mov_address_to_immediate() {
    let instruction = Instruction::new(
        InstructionKind::Mov,
        vec![
            Operand::Address(0x12),
            Operand::Immediate(0x08),
        ],
    );
    match instruction.build() {
        Ok(_) => panic!("Mov to immediate should not be allowed"),
        _ => (),
    }
}

#[test]
fn mov_immediate_to_address() -> Result<(), InstructionBuildError> {
    let instruction = Instruction::new(
        InstructionKind::Mov,
        vec![
            Operand::Immediate(0x34),
            Operand::Address(0x12),
        ],
    );
    let result = instruction.build()?;

    assert_eq!(result, data_block![MOVR_CODE, 0x34, 0x12]);

    Ok(())
}

#[test]
fn mov_immediate_to_immediate() {
    let instruction = Instruction::new(
        InstructionKind::Mov,
        vec![
            Operand::Immediate(0x34),
            Operand::Immediate(0x12),
        ],
    );

    match instruction.build() {
        Ok(_) => panic!("Mov to immediate should not be allowed"),
        _ => (),
    }
}
