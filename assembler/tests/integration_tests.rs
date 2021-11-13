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
fn full_functionality_should_compile() {
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
pow (11) (2) 0x28

add 0x08 (2) 0x08
sub 0x08 (2) 0x10
mul 0x08 (2) 0x18
div 0x08 (2) 0x20
pow 0x08 (2) 0x28
;; TODO: mod would be nice

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

#[test]
fn future_references_should_be_filled_in() {
    let source_code = String::from("jmp :main\n:main\nnop");
    #[rustfmt::skip]
    let mut expected_data_simple: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0xC,
        0, 0, 0, 0, 0, 0, 0, 0x18,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    let mut expected_data: Vec<u8> = CCLASP_SIGNATURE.to_vec();
    expected_data.append(&mut expected_data_simple);

    let compiled_code = clasm::compiling::compile_text(source_code);
    assert_eq!(expected_data, compiled_code);
}

#[test]
#[should_panic]
fn future_references_should_error_if_not_filled() {
    let source_code = String::from("jmp :main");

    clasm::compiling::compile_text(source_code);

    // TODO: below is for after compiled text is changed to error return
    // assert!(clasm::compiling::compile_text(source_code).is_err());
}
