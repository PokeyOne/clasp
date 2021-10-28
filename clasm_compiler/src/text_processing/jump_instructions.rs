//! This module handles instructions that deal with jumping around
//! in the program.

use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::instruction_codes::{JMP_ADDR_CODE, JMP_LIT_CODE};

use super::utility;
use super::{ArgType, Argument, OpProcessError};

pub fn jmp_process(words: Vec<String>) -> Result<Vec<u8>, OpProcessError> {
    println!("jmp: {:?}", &words);

    if words.len() != 2 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Expected one jump location after jmp instructions".to_string()
        ));
    }

    let arg: Argument = match utility::process_arg(&words[1]) {
        Some(value) => value,
        None => return Err(OpProcessError::InvalidArgument)
    };

    let op_code = match arg.arg_type {
        ArgType::Address => JMP_ADDR_CODE,
        ArgType::Literal => JMP_LIT_CODE
    };

    let mut res: Vec<u8> = Vec::new();

    res.append(&mut op_code.to_bytes().to_vec());
    res.append(&mut arg.value.to_bytes().to_vec());

    println!("jmp bytes: {:?}", &res);

    Ok(res)
}
