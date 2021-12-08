use crate::code_generation::InstructionBuilder;

// Most standard library things wont
//       actually be implemented here because they will be single instructions
pub fn construct_std_fn_assembly(name: &str) -> Option<String> {
    match name {
        "println" => Some(println_assembly()),
        _ => None
    }
}

/// If the name provided is the name of a function that maps directly to
/// a single assembly instruction, then this method will return the instruction.
///
/// The returned instruction builder will not have any of the arguments filled
/// in so those should be filled in with the literal values of the expressions
/// following the method call.
pub fn std_call(name: &str) -> Option<InstructionBuilder> {
    match name {
        "print" => {
            Some(InstructionBuilder::new("print".to_string()))
        },
        _ => None
    }
}

// IDEA: make the first 2 or 3 bits of an address be the section. 00 for program
//  01 for program data, 10 for memory, 11 for registers, etc
fn println_assembly() -> String {
    "println:\n\
    print arg0\n\
    print 0x20\n\
    return".to_string()
}