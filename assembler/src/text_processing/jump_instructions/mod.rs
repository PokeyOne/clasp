//! This module handles instructions that deal with jumping around
//! in the program.

#[cfg(test)]
mod tests;

use clasp_common::data_constants::WORD_SIZE;
use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::instruction_codes::{
    CALL_CODE, JMP_ADDR_CODE, JMP_LIT_CODE, RETURN_CODE
};

use super::utility;
use super::{ArgType, Argument, OpProcessError};

pub fn jmp_process(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    println!("jmp: {:?}", &words);

    if words.len() != 2 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Expected one jump location after jmp instructions".to_string()
        ));
    }
    let mut future_label_refs: Vec<(String, u64)> = Vec::new();
    let (arg, future_label_ref) = arg_or_label(&words, 1)?;
    match future_label_ref {
        Some(val) => future_label_refs.push(val),
        None => {}
    };

    let op_code = match arg.arg_type {
        ArgType::Address => JMP_ADDR_CODE,
        ArgType::Literal => JMP_LIT_CODE
    };

    let mut res: Vec<u8> = Vec::new();

    res.append(&mut op_code.to_bytes().to_vec());
    res.append(&mut arg.value.to_bytes().to_vec());

    println!("jmp bytes: {:?}", &res);

    Ok((res, future_label_refs))
}

pub fn return_process(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    println!("return: {:?}", &words);

    if words.len() != 1 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Expected no arguments after return instruction".to_string()
        ));
    }

    Ok((RETURN_CODE.to_bytes().to_vec(), Vec::new()))
}

/// This function handles the call instruction.
///
/// Call instrucitons are used like a jump instruction, but they push all
/// registers onto the stack and push a return address onto the stack. The next
/// return instruction can be used to return to the calling function.
///
/// # Instruction Examples
/// ```asm
/// :some_label
/// ;; do something
/// return
///
/// :main
/// call :some_label
/// ```
pub fn call_process(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    println!("call: {:?}", &words);

    if words.len() != 2 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Expected one label argument after call instruction".to_string()
        ));
    }

    let mut future_label_refs: Vec<(String, u64)> = Vec::new();
    let (arg, future_label_ref) = arg_or_label(&words, 1)?;
    match future_label_ref {
        Some(val) => future_label_refs.push(val),
        None => {}
    };

    let mut res: Vec<u8> = CALL_CODE.to_bytes().to_vec();
    res.append(&mut arg.value.to_bytes().to_vec());

    Ok((res, future_label_refs))
}

// TODO: it would be nice to have this return type as a struct so it could be
//       created and readable easier
pub fn conditional_jump_process(
    words: Vec<String>
) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    // TODO: use ConditionalType to get instruction code and call the jmp_process to process all the arguments and then overwrite instruction codex3
    return Err(OpProcessError::UnimplementedFeature);
}

/// This is essentially the common stuff between call and jmp
fn arg_or_label(
    words: &Vec<String>,
    index: usize
) -> Result<(Argument, Option<(String, u64)>), OpProcessError> {
    match utility::process_arg(&words[index]) {
        None => Err(OpProcessError::InvalidArgument),
        Some(res) => match res.1 {
            Some(flr) => Ok((res.0, Some((flr.0, (index * WORD_SIZE) as u64)))),
            None => Ok(res)
        }
    }
}
