// TODO: In the future the crate should just be called clasm
use clasm_compiler as clasm;
use clasp_common::io::CCLASP_SIGNATURE;

#[test]
fn basic_nop_and_mov_no_registers() {
    let source_code = String::from(
        "nop
nop
mov (0x01) 0x00
mov 0x00 0x08
end"
    );

    let compiled = clasm::compiling::compile_text(source_code);

    let mut expected: Vec<u8> = CCLASP_SIGNATURE.to_vec();
    #[rustfmt::skip]
    let mut simple_expected: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0, // nop
        0, 0, 0, 0, 0, 0, 0, 0, // nop
        0, 0, 0, 0, 0, 0, 0, 8, // mov raw
        0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, // mov
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 8,
        0, 0, 0, 0, 0, 0, 0, 7 // end
    ];
    expected.append(&mut simple_expected);

    assert_eq!(expected, compiled);
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
end"
    .to_string();

    let _compiled = clasm::compiling::compile_text(source_code);
}

#[test]
fn full_functionality() {
    let source_code = "jmp :main
:print
outr ga
jmp gb

:main
nop
nop
mov (0x01) 0x00

add (1) (2) 0x08
sub (4) (2) 0x10
mul (1) (2) 0x18
div (4) (2) 0x20
mod (11) (2) 0x28

add 0x08 (2) 0x08
sub 0x08 (2) 0x10
mul 0x08 (2) 0x18
div 0x08 (2) 0x20
mod 0x08 (2) 0x28

outr 'h'
outr (0x65) ;; e
mov (0x65) 0x00
outr 0x00 ;; e

;; this is the start of a loop
mov (0x00) ga
:loop
add ga (1) ga ;; increment ga
jmp :loop
end"
    .to_string();

    let _compiled = clasm::compiling::compile_text(source_code);
}
