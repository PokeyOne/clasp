use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::NOP_CODE;
use crate::text_processing::program_builder::InstructionBuildError;

/// Builds a NOP instruction. A nop instruction is always one word (8 bytes)
/// long. Will never fail, but returns a result type anyway to keep with the
/// convention of the other build functions.
///
/// # Examples
/// ```
/// # use clasp_assembler::text_processing::program_builder::build_functions::nop;
/// # use clasp_assembler::text_processing::program_builder::InstructionBuildError;
/// let result = nop::nop();
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
/// ```
pub fn nop() -> Result<Vec<u8>, InstructionBuildError> {
    Ok(NOP_CODE.to_bytes().to_vec())
}