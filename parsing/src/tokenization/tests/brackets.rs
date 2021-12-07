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
        vec![Token::CloseCurlyBracket],
    ];

    let calculated: Vec<Vec<Token>> = input.iter().map(|ele| tokenize(&ele).unwrap()).collect();
    assert_eq!(expected, calculated);
}

#[test]
fn test_simple_number_surrounded_by_brackets() {
    let input = "((1))".to_string();
    let expected = vec![
        Token::OpenBracket,
        Token::OpenBracket,
        number_literal!(1),
        Token::CloseBracket,
        Token::CloseBracket,
    ];

    let calculated: Vec<Token> = tokenize(&input).unwrap();
    assert_eq!(expected, calculated);
}
