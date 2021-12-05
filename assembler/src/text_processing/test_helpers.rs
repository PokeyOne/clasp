// TODO: The contents of the string_vec macro have been moved to common/test_helpers.rs.

#[macro_export]
macro_rules! string_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! data_block {
    ( $( $x:expr ),* ) => {
        {
            use clasp_common::data_types::{Word, ByteCollection};
            let mut temp_vec: Vec<u8> = Vec::new();
            $(
                temp_vec.append(&mut ($x as Word).to_bytes().to_vec());
            )*
            temp_vec
        }
    }
}

#[test]
fn string_vec_macro_rule() {
    let result: Vec<String> = string_vec!["this", "is", "a", "test"];

    assert_eq!(4, result.len());
    assert_eq!("this", result[0]);
}

#[test]
fn data_block_macro_size() {
    let data_block = data_block![0, 0];

    assert_eq!(16, data_block.len());
}

#[test]
fn data_block_macro_value() {
    let data_block = data_block![0xFFEEDDCCBBAA9988, 0xFFF];

    assert_eq!(16, data_block.len());

    #[rustfmt::skip]
    let expected_data = vec![
        0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88,
        0, 0, 0, 0, 0, 0, 15, 255
    ];

    assert_eq!(data_block, expected_data);
}
