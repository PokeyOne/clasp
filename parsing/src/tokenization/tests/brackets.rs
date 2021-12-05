use super::*;

use clasp_common::string_vec;

#[test]
fn test_single_brackets() {
    let input = string_vec!["(", ")", "[", "]", "{", "}"];
    let expected = vec![
        vec![Token::OpenBracket],
        vec![Token::CloseBracket],
        vec![Token::OpenSquareBracket],
        vec![Token::CloseSquareBracket],
        vec![Token::OpenCurlyBracket],
        vec![Token::CloseCurlyBracket]
    ];

    let calculated: Vec<Vec<Token>> = input.iter().map(|ele| tokenize(&ele).unwrap()).collect();
    assert_eq!(expected, calculated);
}
