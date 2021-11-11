//! This module handles instructions that deal with jumping around
//! in the program.

use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::instruction_codes::{JMP_ADDR_CODE, JMP_LIT_CODE};

use super::utility;
use super::{ArgType, Argument, OpProcessError};

pub fn jmp_process(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    println!("jmp: {:?}", &words);

    if words.len() != 2 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Expected one jump location after jmp instructions".to_string()
        ));
    }

    let mut future_label_references: Vec<(String, u64)> = Vec::new();

    let arg: Argument = match utility::process_arg(&words[1]) {
        Some(value) => value,
        None => match words[1].chars().nth(0) {
            Some(':') => {
                // add a reference to be filled in later. The location is
                // temporarily 8, which is the location relative to this
                // instruction. After this method call, all the references will
                // be offset by the location of the instruction.
                future_label_references.push((words[1].clone(), 8));
                // The address is temporarily 0 to be filled in later.
                Argument::address(0)
            },
            _ => return Err(OpProcessError::InvalidArgument),
        }
    };

    let op_code = match arg.arg_type {
        ArgType::Address => JMP_ADDR_CODE,
        ArgType::Literal => JMP_LIT_CODE
    };

    let mut res: Vec<u8> = Vec::new();

    res.append(&mut op_code.to_bytes().to_vec());
    res.append(&mut arg.value.to_bytes().to_vec());

    println!("jmp bytes: {:?}", &res);

    // TODO
    Ok((res, future_label_references))
}
