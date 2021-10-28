//! This module handles instructions that deal with writing and reading
//! to and from the console.
//!
//! These include:
//! - outr <some raw ascii byte> : Out put one raw ascii character to the screen

use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::instruction_codes::*;

use super::utility;
use super::{ArgType, Argument, OpProcessError};

pub fn outr_process(words: Vec<String>) -> Result<Vec<u8>, OpProcessError> {
    println!("outr: {:?}", &words);

    if words.len() != 2 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Syntax error, expected 2 arguments on outr instruction".to_string()
        ));
    }

    let argument: Argument = match utility::process_arg(&words[1]) {
        Some(value) => value,
        None => return Err(OpProcessError::InvalidArgument)
    };

    let op_code = match argument.arg_type.clone() {
        ArgType::Address => OUTR_ADDR_CODE,
        ArgType::Literal => OUTR_LIT_CODE
    };

    let mut resulting_byte_code: Vec<u8> = Vec::new();

    resulting_byte_code.append(&mut op_code.to_bytes().to_vec());
    resulting_byte_code.append(&mut argument.value.to_bytes().to_vec());

    println!("outr bytes: {:?}", &resulting_byte_code);

    Ok(resulting_byte_code)
}
