pub mod ast;

#[cfg(test)]
mod tests;

use crate::tokenization::Token;
use ast::{Ast, Expression, Literal};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
pub enum AstConstructionError {
    UnexpectedToken(Token),
    UnexpectedEof
}

// Convenience alias for this file.
type Error = AstConstructionError;

/// Parse a stream of tokens into a meaningful Abstract Syntax Tree.
pub fn parse(tokens: Vec<Token>) -> Result<Ast, Error> {
    // Turn the tokens into a peekable iterator that the rest of the
    // methods can interact with.
    let mut data: Peekable<IntoIter<Token>> = tokens.into_iter().peekable();

    // The root is a series by default.
    let mut exprs: Vec<Expression> = Vec::new();

    // Parse all expressions in the token stream.
    while let Some(expr) = parse_expr(&mut data)? {
        exprs.push(expr)
    }

    // Construct the root series node.
    let result = Ast {
        root: Expression::Series(exprs)
    };

    Ok(result)
}

fn parse_expr(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Option<Expression>, Error> {
    let next_token = match tokens.next() {
        Some(t) => t,
        None => return Ok(None)
    };

    match next_token {
        Token::OpenBracket => parse_series_expr(tokens),
        Token::CloseBracket => Err(Error::UnexpectedToken(Token::CloseBracket)),
        Token::Identifier(s) => Ok(Some(Expression::Ident(s))),
        Token::StringLiteral(s) => Ok(Some(Expression::Literal(Literal::StringLiteral(s))))
    }
}

fn parse_series_expr(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Option<Expression>, Error> {
    let mut exprs = Vec::new();

    loop {
        let next_token = match tokens.peek() {
            Some(t) => t,
            None => return Err(Error::UnexpectedEof)
        };

        match next_token {
            Token::CloseBracket => {
                // Safe to unwrap because just did a peek.
                tokens.next().unwrap();

                return Ok(Some(Expression::Series(exprs)));
            }
            _ => match parse_expr(tokens)? {
                Some(v) => exprs.push(v),
                None => return Err(Error::UnexpectedEof)
            }
        }
    }
}
