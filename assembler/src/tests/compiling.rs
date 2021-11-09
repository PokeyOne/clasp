use crate::compiling;
use clasp_common::io::CCLASP_SIGNATURE;

#[test]
fn single_nop() {
    let mut expected: Vec<u8> = CCLASP_SIGNATURE.to_vec();
    for i in 0..8 {
        expected.push(0);
    }
    let input_text = "nop\n".to_string();
    let calculated = compiling::compile_text(input_text);

    assert_eq!(expected, calculated);
}

#[test]
fn single_mov_with_literal_src() {
    let mut expected: Vec<u8> = CCLASP_SIGNATURE.to_vec();
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
