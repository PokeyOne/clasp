use super::*;
use crate::{string_vec, data_block};

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
