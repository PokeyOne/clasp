use super::cmp_process;
use crate::{data_block, string_vec};
use clasp_common::instruction_constants::instruction_codes::*;

#[test]
fn literal_comparison() {
    let source = string_vec!["cmp", "(1)", "(1)"];
    let expected = data_block![CMP_CODE_CC, 1, 1];

    match cmp_process(source) {
        Err(t) => panic!("process failed: {:?}", t),
        Ok(v) => {
            let (data, flrs) = v;

            assert_eq!(expected, data);
            assert_eq!(0, flrs.len());
        }
    }
}
