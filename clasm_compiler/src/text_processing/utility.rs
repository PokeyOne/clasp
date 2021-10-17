use super::{ArgType, Argument};
use clasp_common::data_types::{ByteCollection, Word};
use hex;

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
/// - `reg` = The value in the register is used. (the name is an address)
/// - `(reg)` = The actual address of the register.
///
/// Essentially anywhere that you see a register name it will be replaced with
/// the address of the register.
pub fn process_arg(val: &str) -> Option<Argument> {
    // TODO: Have a function "get_reg_address" that will return an address if
    //       the string given to it is a register name, but will return None
    //       when not given the name of a register. Then just match it here.

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
        let raw_value_vec: Vec<u8> = match hex::decode(&val[2..]) {
            Ok(vec) => vec,
            Err(err) => panic!("ToHexError: {:?}", err)
        };
        let raw_value: u64 = Word::from_bytes_v(&raw_value_vec);
        return Some(Argument::new(ArgType::Address, raw_value));
    }
    println!("[DEBUG] {} does not start with 0x", val);

    // TODO: Currently can only process hex values. Add:
    //   - register names
    //   - decimal
    return None;
}
