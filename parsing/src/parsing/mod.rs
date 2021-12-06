mod ast;
#[cfg(test)]
mod tests;

use std::vec::IntoIter;
use crate::tokenization::{Token, Literal as TokenLiteral, Symbol};
use ast::{Ast, Expression, Statement, Literal as AstLiteral};

#[derive(Debug, Clone, PartialEq)]
pub enum AstConstructionError {
    UnexpectedToken(Token),
    IteratorEmpty,
    UnexpectedEOF,
    #[allow(dead_code)]
    Unimplemented,
}

pub fn parse_tree(tokens: Vec<Token>) -> Result<Ast, AstConstructionError> {
    println!("Parsing a tree");

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
    println!("Parsing an expression from token: {:?}", &token);

    match &token {
        // force unwrap because we know we are passing in a literal
        Token::Literal(_) => Ok(Expression::Literal(AstLiteral::from_literal_token(token).unwrap())),
        Token::OpenBracket => {
            Ok(Expression::Statement(parse_statement(tokens)?))
        },
        Token::Identifier(val) => Ok(Expression::Identifier(val.clone())),
        _ => Err(AstConstructionError::UnexpectedToken(token.clone()))
    }
}

pub fn parse_statement(tokens: &mut IntoIter<Token>) -> Result<Statement, AstConstructionError> {
    let first_token = tokens.next().ok_or(AstConstructionError::IteratorEmpty)?;
    println!("Parsing a statement from token: {:?}", &first_token);
    let statement_identifier = match first_token {
        Token::Identifier(identifier) => identifier,
        Token::Symbol(sym) => sym.functional_name(),
        Token::Keyword(kw) => kw.functional_name(),
        _ => return Err(AstConstructionError::UnexpectedToken(first_token.clone()))
    };
    println!("Identifier: {}", statement_identifier);

    let mut args: Vec<Expression> = Vec::new();

    loop {
        match parse_expression(tokens) {
            Ok(expr) => args.push(expr),
            Err(err) => match err {
                AstConstructionError::IteratorEmpty => return Err(AstConstructionError::UnexpectedEOF),
                AstConstructionError::UnexpectedToken(Token::CloseBracket) => {
                    break;
                },
                _ => return Err(err)
            }
        };
    }

    Ok(Statement::new(statement_identifier, args))
}
