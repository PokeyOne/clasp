#[cfg(test)]
mod tests;

use crate::token::Token;
use std::str::Chars;

pub fn tokenize(text: String) -> Vec<Token> {
    let result: Vec<Token> = Vec::new();

    let words = text.split_whitespace();

    for word in words {
    }

    return result;
}
