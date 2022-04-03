pub mod ast;

#[cfg(test)]
mod tests;

use crate::tokenization::Token;
use ast::Ast;
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq)]
pub enum AstConstructionError {
    UnexpectedToken(Token),
    IteratorEmpty,
    UnexpectedEOF,
    #[allow(dead_code)]
    Unimplemented
}

pub fn parse_tree(tokens: Vec<Token>) -> Result<Ast, AstConstructionError> {
    todo!()
}
