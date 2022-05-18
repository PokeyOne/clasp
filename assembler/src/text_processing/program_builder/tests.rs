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
    let mut program: Rc<RefCell<ProgramBuilder>> = Rc::new(RefCell::new(program));
    let mut program_builder = program.borrow_mut();
    program_builder.add_instruction(InstructionBuilder::new(InstructionKind::Nop, program.clone()));
    program_builder.add_instruction(InstructionBuilder::new(InstructionKind::Nop, program.clone()));
    program_builder.add_instruction(InstructionBuilder::new(InstructionKind::End, program.clone()));
    let data = program_builder.build_binary()?;
    let expected_data = data_block![
        CCLASP_SIGNATURE_WORD,
        NOP_CODE,
        NOP_CODE,
        END_CODE
    ];
    assert_eq!(data, expected_data);
    Ok(())
}