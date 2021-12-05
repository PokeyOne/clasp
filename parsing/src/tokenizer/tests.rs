use super::tokenize;
use crate::token::{Token, TokenKind, LiteralKind};

#[test]
fn tokenize_boolean_literals() {
    assert_eq!(
        tokenize("true false".to_string()),
        vec![
            Token.new("true", TokenKind::Literal(LiteralKind::Bool(true))),
            Token.new("false", TokenKind::Literal(LiteralKind::Bool(false)))
        ]
    );
}

#[test]
fn tokenize_