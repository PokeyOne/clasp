#[cfg(test)]
mod tests;

use clasp_common::data_constants::WORD_SIZE;
use clasp_common::data_types::ByteCollection;
use clasp_common::instruction_constants::instruction_codes::*;

use super::utility::process_arg;
use super::{ArgType, Argument, OpProcessError};

pub fn general_math(words: Vec<String>) -> Result<(Vec<u8>, Vec<(String, u64)>), OpProcessError> {
    let base_op_code = match words[0].as_str() {
        "add" => ADD_CODE,
        "sub" => SUB_CODE,
        "div" => DIV_CODE,
        "mul" => MUL_CODE,
        "pow" => POW_CODE,
        "cmp" => CMP_CODE,
        _ => panic!("Non math instruction code {:?}", words)
    };

    let (alpha_val, beta_val, dest_addr, flrs) = construct_abd(&words)?;
    let op_code: u64 = base_op_code + math_mod_code(&alpha_val, &beta_val);

    Ok((
        construct_byte_code(op_code, alpha_val, beta_val, dest_addr),
        flrs
    ))
}

fn construct_byte_code(op_code: u64, alpha: Argument, beta: Argument, dest_addr: u64) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    result.append(&mut op_code.to_bytes().to_vec());
    result.append(&mut alpha.value.to_bytes().to_vec());
    result.append(&mut beta.value.to_bytes().to_vec());
    result.append(&mut dest_addr.to_bytes().to_vec());

    result
}

fn math_mod_code(a: &Argument, b: &Argument) -> u64 {
    if a.arg_type == ArgType::Literal {
        if b.arg_type == ArgType::Literal {
            return MATH_MOD_CC;
        } else if b.arg_type == ArgType::Address {
            return MATH_MOD_CA;
        }
    } else {
        if b.arg_type == ArgType::Literal {
            return MATH_MOD_AC;
        } else if b.arg_type == ArgType::Address {
            return MATH_MOD_AA;
        }
    }

    panic!(
        "Could not find math mod code for {:?} and {:?}",
        a.arg_type, b.arg_type
    );
}

fn construct_abd(
    words: &Vec<String>
) -> Result<(Argument, Argument, u64, Vec<(String, u64)>), OpProcessError> {
    validate(&words)?;

    let (alpha_val, mut alpha_flr) = match process_arg(&words[1]) {
        Some(value) => value,
        None => return Err(OpProcessError::InvalidArgument)
    };

    match alpha_flr {
        Some(ref mut val) => val.1 += WORD_SIZE as u64,
        None => {}
    };

    let (beta_val, mut beta_flr) = match process_arg(&words[2]) {
        Some(value) => value,
        None => return Err(OpProcessError::InvalidArgument)
    };

    match beta_flr {
        Some(ref mut val) => val.1 += (WORD_SIZE as u64) * 2,
        None => {}
    };

    let mut dest_flr: Option<(String, u64)> = None;
    let destination_address: u64 = match process_arg(&words[3]) {
        Some(value) => {
            let (arg, maybe_flr) = value;
            dest_flr = maybe_flr;

            match arg.arg_type {
                ArgType::Literal => return Err(OpProcessError::ExpectedAddress),
                ArgType::Address => arg.value()
            }
        }
        None => return Err(OpProcessError::InvalidArgument)
    };

    match dest_flr {
        Some(ref mut val) => val.1 += (WORD_SIZE as u64) * 3,
        None => {}
    };

    let mut flrs: Vec<(String, u64)> = Vec::new();
    match alpha_flr {
        Some(val) => flrs.push(val),
        None => {}
    }
    match beta_flr {
        Some(val) => flrs.push(val),
        None => {}
    }
    match dest_flr {
        Some(val) => flrs.push(val),
        None => {}
    }

    Ok((alpha_val, beta_val, destination_address, flrs))
}

fn validate(words: &Vec<String>) -> Result<(), OpProcessError> {
    // Must be four: instr name, a, b, where put result;
    if words.len() != 4 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Syntax error, expected only 3 arguments for math instruction".to_string()
        ));
    }

    Ok(())
}
