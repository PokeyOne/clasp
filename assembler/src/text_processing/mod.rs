pub mod console_instructions;
pub mod general_instructions;
pub mod jump_instructions;
pub mod math_instructions;
pub mod utility;

pub use console_instructions::*;
pub use general_instructions::*;
pub use jump_instructions::*;
pub use math_instructions::*;

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

/// This is an argument in the assembly and represents either an address to a
/// value, or a literal value. Arguments can also be registers or labels, but
/// in those cases registers should be reduces to the address of the memory
/// mapped register, and labels should also be reduced to the address in the
/// program.
impl Argument {
    fn new(arg_type: ArgType, value: u64) -> Argument {
        Argument {
            arg_type: arg_type,
            value: value
        }
    }

    pub fn is_literal(&self) -> bool {
        match self.arg_type {
            ArgType::Literal => true,
            _ => false
        }
    }

    pub fn is_address(&self) -> bool {
        match self.arg_type {
            ArgType::Address => true,
            _ => false
        }
    }

    pub fn val_ref(&self) -> &u64 {
        &self.value
    }
}
