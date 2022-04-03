pub mod ast;

#[cfg(test)]
mod tests;

use crate::tokenization::Token;
use ast::{Ast, Expression, Literal};
use std::vec::IntoIter;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum AstConstructionError {
    UnexpectedToken(Token),
    UnexpectedEOF,
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

fn parse_expr(
    data: &mut Peekable<IntoIter<Token>>
) -> Result<Option<Expression>, Error> {
    // TODO: Implement this.
    Ok(None)
}
