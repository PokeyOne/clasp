mod ast;
#[cfg(test)]
mod tests;

use std::vec::IntoIter;
use crate::tokenization::{Token, Literal as TokenLiteral};
use ast::{Ast, Expression, Statement, Literal as AstLiteral};

#[derive(Debug, Clone, PartialEq)]
pub enum AstConstructionError {
    UnexpectedToken(Token),
    IteratorEmpty,
    #[allow(dead_code)]
    Unimplemented,
}

pub fn parse_tree(tokens: Vec<Token>) -> Result<Ast, AstConstructionError> {
    let mut expressions: Vec<Expression> = Vec::new();

    let mut iter = tokens.into_iter();

    loop {
        match parse_expression(&mut iter) {
            Ok(expr) => expressions.push(expr),
            Err(err) => match err {
                AstConstructionError::IteratorEmpty => break,
                _ => return Err(err)
            }
        };
    }

    Ok(Ast::new(expressions))
}

/// Parses an expression from the given iterator.
/// Expressions can be either a literal value, or a function call (aka
/// statement).
pub fn parse_expression(tokens: &mut IntoIter<Token>) -> Result<Expression, AstConstructionError> {
    let token = tokens.next().ok_or(AstConstructionError::IteratorEmpty)?;

    // TODO: Implement function calls
    match &token {
        // force unwrap because we know we are passing in a literal
        Token::Literal(_) => Ok(Expression::Literal(AstLiteral::from_literal_token(token).unwrap())),
        _ => Err(AstConstructionError::UnexpectedToken(token.clone()))
    }
}
