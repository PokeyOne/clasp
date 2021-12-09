use super::*;
use crate::{data_block, string_vec};
use clasp_common::io::CCLASP_SIGNATURE_WORD;
use clasp_common::instruction_constants::instruction_codes::*;

#[test]
fn test_empty_program() -> Result<(), InstructionBuildError> {
    let mut program = ProgramBuilder::new();
    let data = program.build_binary()?;
    let expected_data = data_block![
        CCLASP_SIGNATURE_WORD,
        END_CODE
    ];
    assert_eq!(data, expected_data);
    Ok(())
}

#[test]
fn test_nop_program() -> Result<(), InstructionBuildError> {
    let mut program = ProgramBuilder::new();
    program.add_instruction(InstructionBuilder::new(InstructionKind::Nop));
    program.add_instruction(InstructionBuilder::new(InstructionKind::Nop));
    program.add_instruction(InstructionBuilder::new(InstructionKind::End));
    let data = program.build_binary()?;
    let expected_data = data_block![
        CCLASP_SIGNATURE_WORD,
        NOP_CODE,
        NOP_CODE,
        END_CODE
    ];
    assert_eq!(data, expected_data);
    Ok(())
}