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
use crate::tokenization::tokens::{Keyword, Symbol};

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
            match c {
                '(' | ')' | '{' | '}' | '[' | ']' => {
                    // Extra space around brackets to make parsing easier
                    output.push(' ');
                    output.push(c);
                    output.push(' ');
                }
                _ => output.push(c)
            };
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
        match parse_bracket(word) {
            Some(token) => {
                tokens.push(token);
                continue;
            },
            None => {}
        }

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

        match parse_keyword(word) {
            Some(token) => {
                tokens.push(token);
                continue;
            },
            None => {}
        }

        match parse_symbol(word) {
            Some(token) => {
                tokens.push(token);
                continue;
            },
            None => {}
        }

        if is_valid_identifier(word) {
            tokens.push(Token::Identifier(word.to_string()));
        } else {
            return Err(TokenizeError::InvalidToken(word.to_string()));
        }
    }

    return Ok(tokens);
    // IDEA: After tokenizing, have modular layers that cleanup the tokens
    // for example, combining '<' and '=' into '<=', etc.
}

fn is_valid_identifier(word: &str) -> bool {
    let mut i = 0;
    for c in word.chars() {
        if !is_valid_identifier_char(c, i == 0) {
            return false;
        }
        i += 1;
    }

    return true;
}

fn is_valid_identifier_char(c: char, is_first_char: bool) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        '0'..='9' if !is_first_char => true,
        _ => false
    }
}

fn parse_symbol(word: &str) -> Option<Token> {
    match word {
        "+" => Some(Token::Symbol(Symbol::Plus)),
        "-" => Some(Token::Symbol(Symbol::Minus)),
        "*" => Some(Token::Symbol(Symbol::Multiply)),
        "/" => Some(Token::Symbol(Symbol::Divide)),
        "=" => Some(Token::Symbol(Symbol::Equal)),
        "!=" => Some(Token::Symbol(Symbol::NotEqual)),
        "<" => Some(Token::Symbol(Symbol::LessThan)),
        "<=" => Some(Token::Symbol(Symbol::LessThanOrEqual)),
        ">" => Some(Token::Symbol(Symbol::GreaterThan)),
        ">=" => Some(Token::Symbol(Symbol::GreaterThanOrEqual)),
        "!" => Some(Token::Symbol(Symbol::Not)),
        "&&" => Some(Token::Symbol(Symbol::And)),
        "||" => Some(Token::Symbol(Symbol::Or)),
        "." => Some(Token::Symbol(Symbol::Dot)),
        "," => Some(Token::Symbol(Symbol::Comma)),
        ":" => Some(Token::Symbol(Symbol::Colon)),
        ";" => Some(Token::Symbol(Symbol::Semicolon)),
        "?" => Some(Token::Symbol(Symbol::QuestionMark)),
        "|" => Some(Token::Symbol(Symbol::Bar)),
        "&" => Some(Token::Symbol(Symbol::Ampersand)),
        _ => None
    }
}

fn parse_keyword(word: &str) -> Option<Token> {
    match word {
        "fn" => Some(Token::Keyword(Keyword::Fn)),
        _ => None
    }
}

fn parse_bracket(word: &str) -> Option<Token> {
    match word {
        "(" => Some(Token::OpenBracket),
        ")" => Some(Token::CloseBracket),
        "{" => Some(Token::OpenCurlyBracket),
        "}" => Some(Token::CloseCurlyBracket),
        "[" => Some(Token::OpenSquareBracket),
        "]" => Some(Token::CloseSquareBracket),
        _ => None
    }
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
