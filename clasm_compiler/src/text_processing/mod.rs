use clasp_common::instruction_constants::instruction_codes::*;
use hex;
use clasp_common::data_types::{ByteCollection, Word};

#[derive(Debug)]
pub enum OpProcessError {
    WrongNumberOfArguments(String)
}

#[derive(Debug, PartialEq, Eq)]
enum ArgType {
    Literal,
    Address
}

#[derive(Debug)]
struct Argument {
    arg_type: ArgType,
    value: u64
}

impl Argument {
    fn new(arg_type: ArgType, value: u64) -> Argument {
        Argument {
            arg_type: arg_type,
            value: value
        }
    }
}

fn process_arg(val: &str) -> Option<Argument> {
    // This is a literal
    if val.chars().nth(0)? == '(' && val.chars().nth(val.len() - 1)? == ')' {
        // Recursively get this method to process the inside value and then
        // return it with the type swapped to literal
        return match process_arg(&val[1..(val.len() - 1)]) {
            Some(val) => Some(Argument::new(ArgType::Literal, val.value)),
            None => None
        };
    }

    // Hex value
    if val.chars().nth(0)? == '0' && val.chars().nth(1)? == 'x' {
        println!("[DEBUG] processing {}", val);
        let raw_value_vec: Vec<u8> = match hex::decode(&val[2..]) {
            Ok(vec) => vec,
            Err(err) => panic!("ToHexError: {:?}", err)
        };
        let raw_value: u64 = Word::from_bytes_v(&raw_value_vec);
        println!("[DEBUG] got raw_value: {}", raw_value);
        return Some(Argument::new(ArgType::Address, raw_value));
    }
    println!("[DEBUG] {} does not start with 0x", val);

    // TODO: Currently can only process hex values. Add:
    //   - register names
    //   - decimal
    return None;
}

pub fn nop_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("nop: {:?}", &words);

    if words.len() > 1 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Syntax error, unexpected arguments for nop instruction".to_string()
        ));
    }

    Ok(vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8])
}

// TODO Return u8 array to add to buffer
// TODO: Return should be Result<Vec<u8>, String>
pub fn mov_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("mov: {:?}", &words);

    if words.len() != 3 {
        panic!(
            "Syntax error, expected 2 arguments on mov instruction, found {}",
            words.len() - 1
        );
    }

    let origin_arg: Argument = match process_arg(words[1]) {
        Some(value) => value,
        None => panic!("Syntax error, argument 1 is not processable")
    };

    let destination_arg: Argument = match process_arg(words[2]) {
        Some(value) => match value.arg_type {
            ArgType::Address => value,
            ArgType::Literal => panic!("Syntax error, destination must be address")
        },
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

    Ok(resulting_byte_code)
}

