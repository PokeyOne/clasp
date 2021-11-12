use super::{ArgType, Argument};
use clasp_common::data_constants;
use clasp_common::data_types::{ByteCollection, Word};

pub mod numbers;

#[cfg(test)]
mod tests;

/// Given the string token of the raw assembly code instruction argument, this
/// will process that and return an argument object.
///
/// Arguments have 3 main types:
/// - Literal value
/// - Address value
/// - Register name
///
/// An address is written as just a plain number: `34`, `0xF3`, `0b00101011`, or `0o147`
///
/// A literal value is written the same as an address but wrapped in round
/// brackets: `(34)`, `(0xF3)`, `(0b00101011)`, or `(0o147)`
///
/// And finally a register name is just the character string identifier of the
/// register. Similar to before it can take several forms:
/// - `reg` = The actual address of the register.
/// - `(reg)` = The value in the register is used. (the name is an address)
///
/// Essentially anywhere that you see a register name it will be replaced with
/// the address of the register.
pub fn process_arg(val: &str) -> Option<Argument> {
    match data_constants::get_register_address(val) {
        Some(addr) => return Some(Argument::new(ArgType::Address, addr)),
        None => {}
    };

    // This is a literal
    if val.chars().nth(0)? == '(' && val.chars().nth(val.len() - 1)? == ')' {
        // Recursively get this method to process the inside value and then
        // return it with the type swapped to literal
        return match process_arg(&val[1..(val.len() - 1)]) {
            Some(val) => Some(Argument::new(ArgType::Literal, val.value)),
            None => None
        };
    }

    match numbers::parse_number(val) {
        Ok(val) => Some(Argument::address(val)),
        Err(reason) => {
            println!("number {:?} parse error: {}", val, reason);
            None
        }
    }
}
