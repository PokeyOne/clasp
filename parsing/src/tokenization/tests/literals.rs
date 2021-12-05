use super::*;
use super::tokens::*;

#[test]
fn parse_empty_string() -> Result<(), TokenizeError> {
    let input = "".to_string();
    let actual = tokenize(&input)?;
    assert_eq!(0, actual.len());
    Ok(())
}

#[test]
fn parse_single_string() -> Result<(), TokenizeError> {
    let input = "\"test\"".to_string();
    let expected = vec![Token::Literal(Literal::String("test".to_string()))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_multi_word_string() -> Result<(), TokenizeError> {
    let input = "\"test string\"".to_string();
    let expected = vec![Token::Literal(Literal::String("test string".to_string()))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_multi_word_string_with_escaped_newline() -> Result<(), TokenizeError> {
    let input = "\"test\\nstring\"".to_string();
    let expected = vec![Token::Literal(Literal::String("test\nstring".to_string()))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_integer() -> Result<(), TokenizeError> {
    let input = "123".to_string();
    let expected = vec![Token::Literal(Literal::Number(123.0))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_negative_integer() -> Result<(), TokenizeError> {
    let input = "-123".to_string();
    let expected = vec![Token::Literal(Literal::Number(-123.0))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_float() -> Result<(), TokenizeError> {
    let input = "123.456".to_string();
    let expected = vec![Token::Literal(Literal::Number(123.456))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_negative_float() -> Result<(), TokenizeError> {
    let input = "-123.456".to_string();
    let expected = vec![Token::Literal(Literal::Number(-123.456))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_float_with_exponent() -> Result<(), TokenizeError> {
    let input = "123.456e7".to_string();
    let expected = vec![Token::Literal(Literal::Number(123.456e7))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_single_boolean() -> Result<(), TokenizeError> {
    let input = "true".to_string();
    let expected = vec![Token::Literal(Literal::Boolean(true))];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_sequence_of_booleans() -> Result<(), TokenizeError> {
    let input = "true false true".to_string();
    let expected = vec![
        Token::Literal(Literal::Boolean(true)),
        Token::Literal(Literal::Boolean(false)),
        Token::Literal(Literal::Boolean(true))
    ];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn parse_many_literals() -> Result<(), TokenizeError> {
    let input = "123 true false 123.456e7 \"test string\"".to_string();
    let expected = vec![
        Token::Literal(Literal::Number(123.0)),
        Token::Literal(Literal::Boolean(true)),
        Token::Literal(Literal::Boolean(false)),
        Token::Literal(Literal::Number(123.456e7)),
        Token::Literal(Literal::String("test string".to_string()))
    ];
    let actual = tokenize(&input)?;
    assert_eq!(expected, actual);
    Ok(())
}
