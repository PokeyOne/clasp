use clasm_compiler as assembler;
use clasp_runtime_environment as runner;

use runner::memory::{Memory, types::Result::Ok as MemOk};

#[test]
fn basic_linear_program() -> Result<(), String> {
    let source_code = String::from("nop
nop
mov (0x30) 0
outr 0        ;; 0
add (1) 0 0
outr 0        ;; 1
add (1) 0 0
outr 0        ;; 2
add (1) 0 0
outr 0        ;; 3
add (1) 0 0
outr 0        ;; 4
add (1) 0 0
outr 0        ;; 5
add (1) 0 0
outr 0        ;; 6
add (1) 0 0
outr 0        ;; 7
add (1) 0 0
outr 0        ;; 8
add (1) 0 0
outr 0        ;; 9
add (1) 0 0
outr 0        ;; :
outr (0x0A)   ;; \n
end");
    let compiled_code = assembler::compiling::compile_text(source_code);
    let mut program_memory = Memory::new(compiled_code.len() as u64);
    program_memory.writes(0, &compiled_code[8..]);

    let resulting_memory = runner::running::run_program(program_memory, 16)?;

    resulting_memory.debug_dump();

    match resulting_memory.read(0) {
        MemOk(val) => assert_eq!(0x3Au64, val),
        _ => panic!("Could not read mem zero")
    }
    match resulting_memory.read(1) {
        MemOk(val) => assert_eq!(0, val),
        _ => panic!("Could not read mem one")
    }

    Ok(())
}
