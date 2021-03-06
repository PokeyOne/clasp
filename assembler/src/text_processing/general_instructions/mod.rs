use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::instruction_codes::*;

use super::utility::process_arg;
use super::{ArgType, Argument, OpProcessError};

pub fn nop_process(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    println!("nop: {:?}", &words);

    if words.len() > 1 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Syntax error, unexpected arguments for nop instruction".to_string()
        ));
    }

    Ok((vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8], Vec::new()))
}

pub fn end_process(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    println!("end: {:?}", &words);

    Ok((vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 7u8], Vec::new()))
}

pub fn mov_process(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    println!("mov: {:?}", &words);

    // TODO: Convert these panics into OpProcessErrors
    if words.len() != 3 {
        panic!(
            "Syntax error, expected 2 arguments on mov instruction, found {}",
            words.len() - 1
        );
    }

    let mut flrs: Vec<(String, u64)> = Vec::new();

    let origin_arg: Argument = match process_arg(&words[1]) {
        Some(value) => {
            let (arg, flr) = value;

            match flr {
                Some(thing) => flrs.push(thing),
                None => {}
            };

            arg
        }
        None => panic!("Syntax error, argument 1 is not processable")
    };

    let destination_arg: Argument = match process_arg(&words[2]) {
        Some(value) => {
            let (arg, flr) = value;

            match flr {
                Some(thing) => flrs.push(thing),
                None => {}
            };

            match arg.arg_type {
                ArgType::Address => arg,
                ArgType::Literal => panic!("Syntax error, destination must be address")
            }
        }
        None => panic!("Syntax error, argument 2 is not processable")
    };

    let mut resulting_byte_code: Vec<u8> = Vec::new();

    let op_code = if origin_arg.arg_type == ArgType::Address {
        MOV_CODE
    } else {
        MOVR_CODE
    };
    resulting_byte_code.append(&mut op_code.to_bytes().to_vec());

    resulting_byte_code.append(&mut origin_arg.value.to_bytes().to_vec());
    resulting_byte_code.append(&mut destination_arg.value.to_bytes().to_vec());

    println!("mov bytes: {:?}", &resulting_byte_code);

    Ok((resulting_byte_code, flrs))
}
