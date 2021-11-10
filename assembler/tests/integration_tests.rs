// TODO: In the future the crate should just be called clasm
use clasm_compiler as clasm;

#[test]
fn basic_nop_and_mov_no_registers() {
    let source_code = String::from("nop
nop
mov (0x01) 0x00
mov 0x00 0x01
end");

    let compiled = clasm::compiling::compile_text(source_code);
}

#[test]
fn basic_nop_and_mov_with_registers() {
    let source_code = "nop
nop
mov (0x01) 0x00
mov (0x02) ga
mov ga 0x08
mov (ga) 0x10
mov 0x00 0x18
end".to_string();

    let compiled = clasm::compiling::compile_text(source_code);
}
