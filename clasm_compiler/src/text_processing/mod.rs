pub mod console_instructions;
pub mod general_instructions;
pub mod math_instructions;
pub mod jump_instructions;
pub mod utility;

pub use console_instructions::*;
pub use general_instructions::*;
pub use math_instructions::*;
pub use jump_instructions::*;

#[derive(Debug)]
pub enum OpProcessError {
    WrongNumberOfArguments(String),
    InvalidArgument,
    ExpectedAddress,
    #[allow(dead_code)]
    UnimplementedFeature
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
