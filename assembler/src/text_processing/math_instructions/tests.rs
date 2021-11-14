use super::*;
use crate::{data_block, string_vec};
use clasp_common::instruction_constants::*;

#[test]
fn simple_add() {
    let source = string_vec!["add", "(1)", "0x20", "0x20"];
    let expected = data_block![ADD_CODE_CA, 1, 0x20, 0x20];

    match add_process(source.clone()) {
        Err(err) => panic!(
            "Failed to process add instruction {:?} bc: {:?}",
            source, err
        ),
        Ok(ret) => {
            let (data, flrs) = ret;

            assert_eq!(0, flrs.len());
            assert_eq!(data, expected);
        }
    }
}

#[test]
fn label_add() {
    let source = string_vec!["add", ":some_data", ":other_data", ":other_data"];
    let expected = data_block![ADD_CODE_AA, 0, 0, 0];
    let expected_flrs = vec![
        (String::from(":some_data"), 8),
        (String::from(":other_data"), 16),
        (String::from(":other_data"), 24),
    ];

    match add_process(source.clone()) {
        Err(err) => panic!(
            "Failed to process add instruction {:?} bc: {:?}",
            source, err
        ),
        Ok(ret) => {
            let (data, flrs) = ret;

            assert_eq!(data, expected);

            assert_eq!(3, flrs.len());
            assert_eq!(expected_flrs, flrs);
        }
    }
}

#[test]
fn simple_sub() {
    // Technically this would be a runtime error, but should compile
    let source = string_vec!["sub", "(1)", "0x20", "0x20"];
    let expected = data_block![SUB_CODE_CA, 1, 0x20, 0x20];

    match sub_process(source.clone()) {
        Err(err) => panic!(
            "Failed to process sub instruction {:?} bc: {:?}",
            source, err
        ),
        Ok(ret) => {
            let (data, flrs) = ret;

            assert_eq!(0, flrs.len());
            assert_eq!(data, expected);
        }
    }
}

#[test]
fn label_sub() {}

#[test]
fn simple_mul() {
    let source = string_vec!["mul", "(1)", "0x20", "0x20"];
    let expected = data_block![MUL_CODE_CA, 1, 0x20, 0x20];

    match mul_process(source.clone()) {
        Err(err) => panic!(
            "Failed to process mul instruction {:?} bc: {:?}",
            source, err
        ),
        Ok(ret) => {
            let (data, flrs) = ret;

            assert_eq!(0, flrs.len());
            assert_eq!(data, expected);
        }
    }
}

#[test]
fn label_mul() {}

#[test]
fn simple_div() {
    let source = string_vec!["div", "(1)", "0x20", "0x20"];
    let expected = data_block![DIV_CODE_CA, 1, 0x20, 0x20];

    match div_process(source.clone()) {
        Err(err) => panic!(
            "Failed to process div instruction {:?} bc: {:?}",
            source, err
        ),
        Ok(ret) => {
            let (data, flrs) = ret;

            assert_eq!(0, flrs.len());
            assert_eq!(data, expected);
        }
    }
}

#[test]
fn label_div() {}

#[test]
fn simple_pow() {
    let source = string_vec!["pow", "(1)", "0x20", "0x20"];
    let expected = data_block![POW_CODE_CA, 1, 0x20, 0x20];

    match pow_process(source.clone()) {
        Err(err) => panic!(
            "Failed to process pow instruction {:?} bc: {:?}",
            source, err
        ),
        Ok(ret) => {
            let (data, flrs) = ret;

            assert_eq!(0, flrs.len());
            assert_eq!(data, expected);
        }
    }
}

#[test]
fn label_pow() {}
