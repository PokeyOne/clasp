//! This module handles instructions that deal with jumping around
//! in the program.

#[cfg(test)]
mod tests;

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

// TODO: Generalize this to the argument processor
/// This is essentially the common stuff between call and jmp. Possibly worth
/// generalizing up a level so that other instructions can have labels as arguments.
fn arg_or_label(
    words: &Vec<String>,
    index: usize
) -> Result<(Argument, Option<(String, u64)>), OpProcessError> {
    let mut future_label_ref: Option<(String, u64)> = None;

    let arg: Argument = match utility::process_arg(&words[index]) {
        Some(value) => value,
        None => match words[index].chars().nth(0) {
            Some(':') => {
                // add a reference to be filled in later. The location is
                // temporarily 8, which is the location relative to this
                // instruction. After this method call, all the references will
                // be offset by the location of the instruction.
                future_label_ref = Some((words[index].clone(), 8 * index as u64));
                // The address is temporarily 0 to be filled in later.
                Argument::address(0)
            }
            _ => return Err(OpProcessError::InvalidArgument)
        }
    };

    Ok((arg, future_label_ref))
}
