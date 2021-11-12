use super::*;
use crate::string_vec;

#[test]
fn basic_outr_literal_instruction() -> Result<(), OpProcessError> {
    let tokens: Vec<String> = string_vec!["outr", "(0x48)"];

    let (data, future_label_references) = outr_process(tokens)?;

    let expected_data: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0x0A,
        0, 0, 0, 0, 0, 0, 0, 0x48
    ];

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_references.len());

    Ok(())
}

#[test]
fn basic_outr_address_instruction() -> Result<(), OpProcessError> {
    let tokens: Vec<String> = string_vec!["outr", "0x4863"];

    let (data, future_label_references) = outr_process(tokens)?;

    let expected_data: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0x09,
        0, 0, 0, 0, 0, 0, 0x48, 0x63
    ];

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_references.len());

    Ok(())
}

#[test]
fn outr_address_max() -> Result<(), OpProcessError> {
    let tokens: Vec<String> = string_vec!["outr", "0xffffffffffffffff"];

    let (data, future_label_references) = outr_process(tokens)?;

    let expected_data: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0x09,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
    ];

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_references.len());

    Ok(())
}

#[test]
fn outr_address_overflow() {
    let tokens: Vec<String> = string_vec!["outr", "0xffaaffaaffaaffaa1"];

    match outr_process(tokens) {
        Ok(_) => panic!("an argument greater than 64 bits should cause overflow"),
        Err(_) => {}
    }
}

#[test]
fn outr_with_register() -> Result<(), OpProcessError> {
    let tokens: Vec<String> = string_vec!["outr", "ga"];

    let (data, future_label_references) = outr_process(tokens)?;

    let expected_data: Vec<u8> = vec![
        0, 0, 0, 0, 0, 0, 0, 0x09,
        0x80, 0, 0, 0, 0, 0, 0, 0
    ];

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_references.len());

    Ok(())
}
