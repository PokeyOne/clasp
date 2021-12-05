#[cfg(test)]
mod tests;

mod tokens;

use tokens::{
    Token,
    Literal,
};
use crate::boolean_literal;
use crate::string_literal;
use crate::number_literal;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizeError {
    InvalidToken(String)
}

fn preprocess_strings(input: &String) -> (Vec<String>, String) {
    let mut strings = Vec::new();
    let mut output = String::new();
    let mut string_literal_buffer = String::new();
    let mut in_string_literal = false;
    let mut last_char = ' ';
    for c in input.chars() {
        if c == '"' && last_char != '\\' {
            if in_string_literal {
                strings.push(string_literal_buffer.clone());
                string_literal_buffer.clear();
                in_string_literal = false;
                // Extra space around literals to make parsing easier
                output.push_str(&format!(" \"{}\" ", strings.len() - 1));
            } else {
                in_string_literal = true;
            }
        } else if in_string_literal {
            if last_char == '\\' {
                string_literal_buffer.pop();
                match c {
                    'n' => string_literal_buffer.push('\n'),
                    'r' => string_literal_buffer.push('\r'),
                    't' => string_literal_buffer.push('\t'),
                    '\\' => string_literal_buffer.push('\\'),
                    '"' => string_literal_buffer.push('"'),
                    _ => string_literal_buffer.push(c),
                }
            } else {
                string_literal_buffer.push(c);
            }
        } else {
            output.push(c);
        }

        last_char = c;
    }
    if in_string_literal {
        panic!("Unclosed string literal");
    }
    (strings, output)
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens: Vec<Token> = Vec::new();
    let (strings, input) = preprocess_strings(input);

    for word in input.split_whitespace() {
        match parse_string(word) {
            Some(index) => {
                tokens.push(string_literal!(strings[index]));
                continue;
            },
            None => {}
        }

        match parse_bool(word) {
            Some(token) => {
                tokens.push(token);
                continue;
            },
            None => {}
        }

        match word.parse::<f64>() {
            Ok(number) => {
                tokens.push(number_literal!(number));
                continue;
            },
            Err(_) => {}
        }

        return Err(TokenizeError::InvalidToken(word.to_string()));
    }

    return Ok(tokens);
}

fn parse_string(word: &str) -> Option<usize> {
    if !(word.starts_with('"') && word.ends_with('"')) {
        return None;
    }

    word[1..(word.len() -1)].parse::<usize>().ok()
}

fn parse_bool(word: &str) -> Option<Token> {
    match word {
        "true" => Some(boolean_literal!(true)),
        "false" => Some(boolean_literal!(false)),
        _ => None,
    }
}