use clasp_common::instruction_constants::instruction_codes::*;
use clasp_common::data_types::ByteCollection;

use super::{OpProcessError, ArgType, Argument};
use super::utility::process_arg;

pub fn add_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("add: {:?}", &words);

    let (alpha_val, beta_val, dest_addr) = construct_abd(&words)?;
    let op_code: u64 = ADD_CODE + math_mod_code(&alpha_val, &beta_val);

    Ok(construct_byte_code(op_code, alpha_val, beta_val, dest_addr))
}

pub fn sub_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("sub: {:?}", &words);

    let (alpha_val, beta_val, dest_addr) = construct_abd(&words)?;
    let op_code: u64 = SUB_CODE + math_mod_code(&alpha_val, &beta_val);

    Ok(construct_byte_code(op_code, alpha_val, beta_val, dest_addr))
}

pub fn mul_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("mul: {:?}", &words);

    let (alpha_val, beta_val, dest_addr) = construct_abd(&words)?;
    let op_code: u64 = SUB_CODE + math_mod_code(&alpha_val, &beta_val);

    Ok(construct_byte_code(op_code, alpha_val, beta_val, dest_addr))
}

pub fn div_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("div: {:?}", &words);

    let (alpha_val, beta_val, dest_addr) = construct_abd(&words)?;
    let op_code: u64 = SUB_CODE + math_mod_code(&alpha_val, &beta_val);

    Ok(construct_byte_code(op_code, alpha_val, beta_val, dest_addr))
}

pub fn pow_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("pow: {:?}", &words);

    let (alpha_val, beta_val, dest_addr) = construct_abd(&words)?;
    let op_code: u64 = SUB_CODE + math_mod_code(&alpha_val, &beta_val);

    Ok(construct_byte_code(op_code, alpha_val, beta_val, dest_addr))
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

    panic!("Could not find math mod code for {:?} and {:?}", a.arg_type, b.arg_type);
}

fn construct_abd(words: &Vec<&str>) -> Result<(Argument, Argument, u64), OpProcessError> {
    validate(&words)?;

    let alpha_val: Argument = match process_arg(words[1]) {
        Some(value) => value,
        None => return Err(OpProcessError::InvalidArgument)
    };

    let beta_val: Argument = match process_arg(words[2]) {
        Some(value) => value,
        None => return Err(OpProcessError::InvalidArgument)
    };

    let destination_address: u64 = match process_arg(words[3]) {
        Some(value) => match value.arg_type {
            ArgType::Literal => return Err(OpProcessError::ExpectedAddress),
            ArgType::Address => value.value
        },
        None => return Err(OpProcessError::InvalidArgument)
    };

    Ok((alpha_val, beta_val, destination_address))
}

fn validate(words: &Vec<&str>) -> Result<(), OpProcessError> {
    // Must be four: instr name, a, b, where put result;
    if (*words).len() != 4 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Syntax error, expected only 2 arguments for add instruction".to_string()
        ));
    }

    Ok(())
}
