use super::*;
use super::tokens::*;

#[test]
fn parse_empty_string() {
    let input = "".to_string();
    let actual = tokenize(&input);
    assert_eq!(0, actual.len());
}

#[test]
fn parse_single_string() {
    let input = "\"test\"".to_string();
    let expected = vec![Token::Literal(Literal::String("test".to_string()))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_multi_word_string() {
    let input = "\"test string\"".to_string();
    let expected = vec![Token::Literal(Literal::String("test string".to_string()))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_multi_word_string_with_escaped_newline() {
    let input = "\"test\\nstring\"".to_string();
    let expected = vec![Token::Literal(Literal::String("test\nstring".to_string()))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_integer() {
    let input = "123".to_string();
    let expected = vec![Token::Literal(Literal::Number(123.0))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_negative_integer() {
    let input = "-123".to_string();
    let expected = vec![Token::Literal(Literal::Number(-123.0))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_float() {
    let input = "123.456".to_string();
    let expected = vec![Token::Literal(Literal::Number(123.456))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_negative_float() {
    let input = "-123.456".to_string();
    let expected = vec![Token::Literal(Literal::Number(-123.456))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_float_with_exponent() {
    let input = "123.456e7".to_string();
    let expected = vec![Token::Literal(Literal::Number(123.456e7))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_single_boolean() {
    let input = "true".to_string();
    let expected = vec![Token::Literal(Literal::Boolean(true))];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_sequence_of_booleans() {
    let input = "true false true t f".to_string();
    let expected = vec![
        Token::Literal(Literal::Boolean(true)),
        Token::Literal(Literal::Boolean(false)),
        Token::Literal(Literal::Boolean(true)),
        Token::Literal(Literal::Boolean(true)),
        Token::Literal(Literal::Boolean(false))
    ];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}

#[test]
fn parse_many_literals() {
    let input = "123 true false 123.456e7 \"test string\"".to_string();
    let expected = vec![
        Token::Literal(Literal::Number(123.0)),
        Token::Literal(Literal::Boolean(true)),
        Token::Literal(Literal::Boolean(false)),
        Token::Literal(Literal::Number(123.456e7)),
        Token::Literal(Literal::String("test string".to_string()))
    ];
    let actual = tokenize(&input);
    assert_eq!(expected, actual);
}
