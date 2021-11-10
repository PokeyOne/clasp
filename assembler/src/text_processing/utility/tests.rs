use crate::text_processing::utility;
use clasp_common::data_constants as dcs;

#[test]
fn process_register_arg() {
    let arg = utility::process_arg("ga");

    match arg {
        None => panic!("Expected arg to be some"),
        Some(value) => {
            assert!(value.is_address());
            assert_eq!(*value.val_ref(), dcs::GA_LOC);
        }
    }
}

#[test]
fn process_register_arg_should_be_none() {
    let arg = utility::process_arg("basphemy");

    assert!(arg.is_none());
}

#[test]
fn process_arg_address_hex() {
    let arg = utility::process_arg("0x4f");
    let other_arg = utility::process_arg("0x4F");

    match arg {
        None => panic!("0x4f should produce some value"),
        Some(arg_value) => {
            assert!(arg_value.is_address());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
        }
    }

    match other_arg {
        None => panic!("0x4F should produce some value"),
        Some(arg_value) => {
            assert!(arg_value.is_address());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
        }
    }
}

#[test]
fn process_arg_literal_hex() {
    let arg = utility::process_arg("(0x4f)");
    let other_arg = utility::process_arg("(0x4F)");

    match arg {
        None => panic!("0x4f should produce some value"),
        Some(arg_value) => {
            assert!(arg_value.is_literal());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
        }
    }

    match other_arg {
        None => panic!("0x4F should produce some value"),
        Some(arg_value) => {
            assert!(arg_value.is_literal());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
        }
    }
}

#[test]
#[ignore = "not implmented yet"]
fn process_arg_literal_decimal() {
    let arg = utility::process_arg("(128)");

    match arg {
        None => panic!("0x4f should produce some value"),
        Some(arg_value) => {
            assert!(arg_value.is_literal());
            assert_eq!(*arg_value.val_ref(), 128u64);
        }
    }
}

#[test]
#[ignore = "not implmented yet"]
fn process_arg_address_decimal() {
    let arg = utility::process_arg("128");

    match arg {
        None => panic!("0x4f should produce some value"),
        Some(arg_value) => {
            assert!(arg_value.is_address());
            assert_eq!(*arg_value.val_ref(), 128u64);
        }
    }
}
