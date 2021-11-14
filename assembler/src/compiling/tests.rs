use crate::{compiling, data_block};
use clasp_common::io::{CCLASP_SIGNATURE, CCLASP_SIGNATURE_WORD};

#[test]
fn single_nop() {
    let mut expected: Vec<u8> = CCLASP_SIGNATURE.to_vec();
    for _i in 0..8 {
        expected.push(0);
    }
    let input_text = "nop\n".to_string();
    let calculated = compiling::compile_text(input_text);

    assert_eq!(expected, calculated);
}

#[test]
fn single_mov_with_literal_src() {
    let mut expected: Vec<u8> = CCLASP_SIGNATURE.to_vec();
    #[rustfmt::skip]
    let mut simple_expected: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 8,
        0, 0, 0, 0, 0, 0, 255, 255,
        0, 0, 0, 0, 0, 0, 0, 0x20
    ];
    expected.append(&mut simple_expected);
    let input_text = "mov (0xFFFF) 0x20".to_string();
    let calculated = compiling::compile_text(input_text);

    assert_eq!(expected, calculated);
}

#[test]
fn single_mov_with_address_src() {
    let mut expected: Vec<u8> = CCLASP_SIGNATURE.to_vec();
    #[rustfmt::skip]
    let mut simple_expected: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 255, 255,
        0, 0, 0, 0, 0, 0, 0, 0x20
    ];
    expected.append(&mut simple_expected);
    let input_text = "mov 0xFFFF 0x20".to_string();
    let calculated = compiling::compile_text(input_text);

    assert_eq!(expected, calculated);
}

#[test]
fn nop_with_comments() {
    let mut expected: Vec<u8> = CCLASP_SIGNATURE.to_vec();
    #[rustfmt::skip]
    let mut simple_expected: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0
    ];
    expected.append(&mut simple_expected);
    let input_text = ";; this line is a comment\nnop\n;; haha nope\nnop ;; with comment at end of line\nnop\n;; comment at end of the file\n;; haha, a multiline comment\n\n\n;; blah".to_string();
    let calculated = compiling::compile_text(input_text);

    assert_eq!(expected, calculated);
}

#[test]
fn single_call() {
    let input_text = "nop
:main
call :main
end"
    .to_string();
    let expected: Vec<u8> = data_block![CCLASP_SIGNATURE_WORD, 0x00, 0x0D, 0x10, 0x07];

    let calculated = compiling::compile_text(input_text);

    assert_eq!(expected, calculated);
}

#[test]
fn single_return() {
    let input_text = "return".to_string();
    let expected: Vec<u8> = data_block![CCLASP_SIGNATURE_WORD, 0x0E];

    let calculated = compiling::compile_text(input_text);

    assert_eq!(expected, calculated);
}

// TODO: #3 - more tests for all instructions
