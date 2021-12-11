use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::END_CODE;
use crate::text_processing::program_builder::InstructionBuildError;

/// Builds a END instruction. An end instruction is always one word (8 bytes)
/// long. Will never fail, but returns a result type anyway to keep with the
/// convention of the other build functions.
///
/// # Examples
/// ```
/// # use clasp_assembler::text_processing::program_builder::build_functions::end;
/// # use clasp_assembler::text_processing::program_builder::InstructionBuildError;
/// # use clasp_common::instruction_constants::END_CODE;
/// # use clasp_common::data_types::ByteCollection;
/// let result = end();
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), END_CODE.to_bytes().to_vec());
/// ```
pub fn end() -> Result<Vec<u8>, InstructionBuildError> {
    Ok(END_CODE.to_bytes().to_vec())
}