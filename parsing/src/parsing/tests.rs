use super::*;
use crate::tokenization::{self, Literal as TokenLiteral};

#[test]
fn parse_single_number_literal() -> Result<(), AstConstructionError> {
    let input = vec![Token::Literal(TokenLiteral::Number(1.0))];
    let expected = Ast::new(vec![Expression::Literal(AstLiteral::Number(1.0))]);

    assert_eq!(expected, parse_tree(input)?);

    Ok(())
}

#[test]
fn parse_series_of_number_literals() -> Result<(), AstConstructionError> {
    let input = vec![
        Token::Literal(TokenLiteral::Number(1.0)),
        Token::Literal(TokenLiteral::Number(2.0)),
        Token::Literal(TokenLiteral::Number(3.0)),
    ];
    let expected = Ast::new(vec![
        Expression::Literal(AstLiteral::Number(1.0)),
        Expression::Literal(AstLiteral::Number(2.0)),
        Expression::Literal(AstLiteral::Number(3.0)),
    ]);

    assert_eq!(expected, parse_tree(input)?);

    Ok(())
}

#[test]
fn parse_addition_expression() -> Result<(), AstConstructionError> {
    let input =
        tokenization::tokenize(&"(+ 1 2)".to_string()).expect("Failed to tokenize test input");
    let expected = Ast::new(vec![Expression::Statement(Statement::new(
        // Parsing the tree should not map the symbol to a token, that
        // is done by the compiler.
        "+".to_string(),
        vec![
            Expression::Literal(AstLiteral::Number(1.0)),
            Expression::Literal(AstLiteral::Number(2.0)),
        ]
    ))]);

    assert_eq!(expected, parse_tree(input)?);

    Ok(())
}

#[test]
fn parse_named_addition_expression() -> Result<(), AstConstructionError> {
    let input =
        tokenization::tokenize(&"(add 1 2)".to_string()).expect("Failed to tokenize test input");
    let expected = Ast::new(vec![Expression::Statement(Statement::new(
        // Parsing the tree should not map the symbol to a token, that
        // is done by the compiler.
        "add".to_string(),
        vec![
            Expression::Literal(AstLiteral::Number(1.0)),
            Expression::Literal(AstLiteral::Number(2.0)),
        ]
    ))]);

    assert_eq!(expected, parse_tree(input)?);

    Ok(())
}

#[test]
fn parse_fn_definition() -> Result<(), AstConstructionError> {
    let input = tokenization::tokenize(&"(fn main (println \"Hello, World!\"))".to_string())
        .expect("Failed to tokenize test input");
    let expected = Ast::new(vec![Expression::Statement(Statement::new(
        "fn".to_string(),
        vec![
            Expression::Identifier("main".to_string()),
            Expression::Statement(Statement::new(
                "println".to_string(),
                vec![Expression::Literal(AstLiteral::String(
                    "Hello, World!".to_string()
                ))]
            )),
        ]
    ))]);

    assert_eq!(expected, parse_tree(input)?);

    Ok(())
}

#[test]
fn parse_complex_math_expression() -> Result<(), AstConstructionError> {
    let input = tokenization::tokenize(&"(+ (+ 1 2) 2 (* 5 -6.3 (^ 2 3)))".to_string())
        .expect("Failed to tokenize test input");
    let expected = Ast::new(vec![Expression::Statement(Statement::new(
        "+".to_string(),
        vec![
            Expression::Statement(Statement::new(
                "+".to_string(),
                vec![
                    Expression::Literal(AstLiteral::Number(1.0)),
                    Expression::Literal(AstLiteral::Number(2.0)),
                ]
            )),
            Expression::Literal(AstLiteral::Number(2.0)),
            Expression::Statement(Statement::new(
                "*".to_string(),
                vec![
                    Expression::Literal(AstLiteral::Number(5.0)),
                    Expression::Literal(AstLiteral::Number(-6.3)),
                    Expression::Statement(Statement::new(
                        "^".to_string(),
                        vec![
                            Expression::Literal(AstLiteral::Number(2.0)),
                            Expression::Literal(AstLiteral::Number(3.0)),
                        ]
                    )),
                ]
            )),
        ]
    ))]);

    assert_eq!(expected, parse_tree(input)?);

    Ok(())
}
