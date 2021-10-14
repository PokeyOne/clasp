pub mod utility;
pub mod general_instructions;
pub mod console_instructions;

pub use general_instructions::*;
pub use console_instructions::*;

#[derive(Debug)]
pub enum OpProcessError {
    WrongNumberOfArguments(String),
    InvalidArgument
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArgType {
    Literal,
    Address
}

#[derive(Debug)]
pub struct Argument {
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

