use super::*;
use crate::{data_block, string_vec};

#[test]
fn basic_outr_literal_instruction() -> Result<(), OpProcessError> {
    let tokens: Vec<String> = string_vec!["outr", "(0x48)"];
    let expected_data: Vec<u8> = data_block![0xA, 0x48];

    let (data, future_label_references) = outr_process(tokens)?;

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_references.len());

    Ok(())
}

#[test]
fn basic_outr_address_instruction() -> Result<(), OpProcessError> {
    let tokens: Vec<String> = string_vec!["outr", "0x4863"];
    let expected_data = data_block![0x09, 0x4863];

    let (data, future_label_references) = outr_process(tokens)?;

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_references.len());

    Ok(())
}

#[test]
fn outr_address_max() -> Result<(), OpProcessError> {
    let tokens: Vec<String> = string_vec!["outr", "0xffffffffffffffff"];
    let expected_data = data_block![9, 0xFFFF_FFFF_FFFF_FFFF];

    let (data, future_label_references) = outr_process(tokens)?;

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
    let expected_data = data_block![9, 0x8000_0000_0000_0000];

    let (data, future_label_references) = outr_process(tokens)?;

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_references.len());

    Ok(())
}
