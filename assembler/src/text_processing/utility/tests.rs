use crate::text_processing::utility;
use clasp_common::data_constants as dcs;

#[test]
fn process_register_arg() {
    let arg = utility::process_arg("ga");

    match arg {
        None => panic!("Expected arg to be some"),
        Some(tup) => {
            let (value, flr) = tup;
            assert!(value.is_address());
            assert_eq!(*value.val_ref(), dcs::GA_LOC);
            assert!(flr.is_none());
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
        Some(tup) => {
            let (arg_value, flr) = tup;
            assert!(arg_value.is_address());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
            assert!(tup.is_none());
        }
    }

    match other_arg {
        None => panic!("0x4F should produce some value"),
        Some(tup) => {
            let (arg_value, flr) = tup;
            assert!(arg_value.is_address());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
            assert!(tup.is_none());
        }
    }
}

#[test]
fn process_arg_literal_hex() {
    let arg = utility::process_arg("(0x4f)");
    let other_arg = utility::process_arg("(0x4F)");

    match arg {
        None => panic!("(0x4f) should produce some value"),
        Some(tup) => {
            let (arg_value, flr) = tup;
            assert!(arg_value.is_literal());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
            assert!(tup.is_none());
        }
    }

    match other_arg {
        None => panic!("(0x4F) should produce some value"),
        Some(tup) => {
            let (arg_value, flr) = tup;
            assert!(arg_value.is_literal());
            assert_eq!(*arg_value.val_ref(), 0x4Fu64);
            assert!(tup.is_none());
        }
    }
}

#[test]
fn process_arg_literal_decimal() {
    let arg = utility::process_arg("(128)");

    match arg {
        None => panic!("(128) should produce some value"),
        Some(tup) => {
            let (arg_value, flr) = tup;
            assert!(arg_value.is_literal());
            assert_eq!(*arg_value.val_ref(), 128u64);
            assert!(tup.is_none());
        }
    }
}

#[test]
fn process_arg_address_decimal() {
    let arg = utility::process_arg("128");

    match arg {
        None => panic!("128 should produce some value"),
        Some(tup) => {
            let (arg_value, flr) = tup;
            assert!(arg_value.is_address());
            assert_eq!(*arg_value.val_ref(), 128u64);
            assert!(tup.is_none());
        }
    }
}

#[test]
fn process_arg_label() {
    let arg = utility::process_arg(":label");

    match arg {
        None => panic!(":label should produce some value"),
        Some(tup) => {
            let (val, flr) = tup;
            assert!(val.is_literal());
            assert_eq!(val.value(), 0);
            assert_eq!(1, flr.len());
            assert_eq!((String::from(":label"), 0), flr.unwrap());
        }
    }
}
