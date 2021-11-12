pub mod ast;
pub mod string_tracker;
pub mod token;

use token::Token;

pub fn tokenize_text(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars();

    let mut whitespace_mode: Option<String> = None;
    let _identifier: Option<String> = None;
    let mut string: Option<String> = None;
    let _number: Option<String> = None;

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            if whitespace_mode.is_none() {
                whitespace_mode = Some(String::new());
            }

            whitespace_mode.as_mut().unwrap().push(c);
            continue;
        } else if whitespace_mode.is_some() {
            tokens.push(Token::Whitespace(whitespace_mode.unwrap()));
            whitespace_mode = None;
            continue;
        }

        match c {
            '(' => tokens.push(Token::OpeningBracket),
            ')' => tokens.push(Token::ClosingBracket),
            '[' => tokens.push(Token::OpeningSquareBracket),
            ']' => tokens.push(Token::ClosingSquareBracket),
            '\'' => tokens.push(Token::KeyStarter),
            _ => {
                if c == '"' {
                    if string.is_some() {
                        tokens.push(Token::StringLiteral(string.unwrap().clone()));
                        string = None;
                    } else {
                        string = Some(String::new())
                    }

                    continue;
                }

                match string {
                    Some(ref mut st) => st.push(c),
                    None => {}
                }

                // TODO: if alpha, then it's an identifier, if number, then it's a number, if double quote, then it's a string
                if c.is_alphabetic() {
                    // TODO
                }
            }
        }
    }

    return tokens;
}
