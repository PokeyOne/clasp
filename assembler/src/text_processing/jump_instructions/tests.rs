use super::*;
use crate::{data_block, string_vec};

#[test]
fn basic_jmp_label() -> Result<(), OpProcessError> {
    let tokens = string_vec!["jmp", ":main"];
    let expected_data: Vec<u8> = data_block![11, 0];

    let (data, future_label_refs) = jmp_process(tokens)?;

    assert_eq!(expected_data, data);
    assert_eq!(1, future_label_refs.len());
    assert_eq!((":main".to_string(), 8), future_label_refs[0]);

    Ok(())
}

#[test]
fn jump_with_wrong_number_of_args() {
    assert!(jmp_process(string_vec!["jmp", ":main", ":function"]).is_err());
    assert!(jmp_process(string_vec!["jmp"]).is_err());
}

#[test]
fn jump_with_address() -> Result<(), OpProcessError> {
    let tokens = string_vec!["jmp", "0x18"];
    let expected_data: Vec<u8> = data_block![11, 0x18];

    let (data, future_label_refs) = jmp_process(tokens)?;

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_refs.len());

    Ok(())
}

#[test]
fn test_call() -> Result<(), OpProcessError> {
    let tokens = string_vec!["call", "0x18"];
    let expected_data: Vec<u8> = data_block![CALL_CODE, 0x18];

    let (data, future_label_refs) = call_process(tokens)?;

    assert_eq!(expected_data, data);
    assert_eq!(0, future_label_refs.len());

    Ok(())
}

#[test]
fn test_call_with_label() -> Result<(), OpProcessError> {
    let tokens = string_vec!["call", ":main"];
    let expected_data: Vec<u8> = data_block![CALL_CODE, 0];

    let (data, future_label_refs) = call_process(tokens)?;

    assert_eq!(expected_data, data);
    assert_eq!(1, future_label_refs.len());
    assert_eq!((":main".to_string(), 8), future_label_refs[0]);

    Ok(())
}
