use super::*;

// TODO
#[test]
fn tokenize_simple_addition_statement() -> Result<(), TokenizeError> {
    let input_a = "(+ 1 2)".to_string();
    let input_b = "(add 1 2)".to_string();
    let expected_a = vec![
        Token::OpenBracket,
        Token::Symbol(Symbol::Plus),
        Token::Literal(Literal::Number(1.0)),
        Token::Literal(Literal::Number(2.0)),
        Token::CloseBracket
    ];
    let expected_b = vec![
        Token::OpenBracket,
        Token::Identifier("add".to_string()),
        Token::Literal(Literal::Number(1.0)),
        Token::Literal(Literal::Number(2.0)),
        Token::CloseBracket
    ];

    assert_eq!(expected_a, tokenize(&input_a)?);
    assert_eq!(expected_b, tokenize(&input_b)?);

    Ok(())
}