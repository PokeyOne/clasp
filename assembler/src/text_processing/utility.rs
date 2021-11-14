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
///
/// The return value of the function is None if argument couldn't be parsed.
/// If the argument can be parsed then the argument value is returned with the
/// appropriate type. The second part of the returned tuple is another tuple
/// that represents the future label reference so that it can be filled in
/// later.
pub fn process_arg(val: &str) -> Option<(Argument, Option<(String, u64)>)> {
    match data_constants::get_register_address(val) {
        Some(addr) => return Some((Argument::new(ArgType::Address, addr), None)),
        None => {}
    };

    // This is a literal
    if val.chars().nth(0) == Some('(') && val.chars().nth(val.len() - 1) == Some(')') {
        // Recursively get this method to process the inside value and then
        // return it with the type swapped to literal
        return match process_arg(&val[1..(val.len() - 1)]) {
            Some(val) => Some((Argument::new(ArgType::Literal, val.0.value), val.1)),
            None => None
        };
    }

    // Detect a label argument
    if val.chars().nth(0) == Some(':') {
        // Temp zero to be filled later.
        let arg = Argument::literal(0);
        // zero is the offset, which is zero relative to what this function
        // is written around.
        let future_label_ref = (val.to_string(), 0);
        return Some((arg, Some(future_label_ref)));
    }

    // Straight-up number
    match numbers::parse_number(val) {
        Ok(val) => Some((Argument::address(val), None)),
        Err(reason) => {
            println!("number {:?} parse error: {}", val, reason);
            None
        }
    }
}
