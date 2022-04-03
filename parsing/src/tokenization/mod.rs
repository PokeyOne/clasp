#[cfg(test)]
mod tests;

mod tokens;

pub use tokens::Token;

use std::str::Chars;
use std::iter::Peekable;

use Token::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenError {
    line: usize,
    col: usize,
    kind: ErrorKind
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenErrorKind {
    UnexpectedChar(char)
}

// Quick aliases for inside this module.
type Error = TokenError;
type ErrorKind = TokenErrorKind;

/// The state of a tokenizer
#[derive(Debug, Clone, PartialEq)]
enum State {
    /// A generic state that delegates responsibilities elsewhere.
    Delegate,
    Identifier(String),
    Whitespace,
    /// Tokenization done.
    Done
}

impl Default for State {
    fn default() -> State {
        State::Delegate
    }
}

/// A private structure that holds certain fields to track tokenization.
struct Tokenizer<'a> {
    /// The iterator over the data characters.
    data: Peekable<Chars<'a>>,
    /// The resulting tokens.
    tokens: Vec<Token>,
    /// The line number in the source code starting from 1.
    line: usize,
    /// The column number in the source code starting from 0.
    col: usize
}

impl<'a> Tokenizer<'a> {
    /// Construct a new tokenizer object from the data provided.
    ///
    /// # Lifetimes
    /// This structure can not live longer than the pointer the string.
    pub fn new(data: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            data: data.chars().peekable(),
            tokens: Vec::new(),
            line: 1,
            col: 1
        }
    }

    /// Consume this tokenizer and return the tokens if successful.
    pub fn tokenize(mut self) -> Result<Vec<Token>, Error> {
        let mut state = State::default();

        while state != State::Done {
            state = match state {
                State::Delegate => self.delegate()?,
                State::Identifier(s) => self.identifier(s)?,
                State::Whitespace => self.whitespace()?,
                State::Done => unreachable!()
            }
        }

        Ok(self.tokens)
    }

    fn delegate(&mut self) -> Result<State, Error> {
        let next = match self.peek() {
            Some(c) => c,
            None => return Ok(State::Done)
        };

        match next {
            '(' => {
                self.tokens.push(Token::OpenBracket);

                self.skip();
                Ok(State::Delegate)
            }
            ')' => {
                self.tokens.push(Token::CloseBracket);

                self.skip();
                Ok(State::Delegate)
            }
            c if c.is_whitespace() => {
                Ok(State::Whitespace)
            }
            c if is_valid_identifier_char(c, true) => {
                Ok(State::Identifier(String::new()))
            }
            _ => Err(self.error(ErrorKind::UnexpectedChar(next)))
        }
    }

    fn identifier(&mut self, mut ident: String) -> Result<State, Error> {
        let is_first_char = ident.len() == 0;

        match self.peek() {
            Some(c) if is_valid_identifier_char(c, is_first_char) => {
                self.skip();
                ident.push(c);
                Ok(State::Identifier(ident))
            }
            _ => {
                if ident.len() > 0 {
                    self.tokens.push(Identifier(ident));
                }

                Ok(State::Delegate)
            }
        }
    }

    fn whitespace(&mut self) -> Result<State, Error> {
        match self.peek() {
            Some(c) if c.is_whitespace() => {
                self.skip();
                Ok(State::Whitespace)
            }
            _ => Ok(State::Delegate)
        }
    }

    /// Take a little peeksy at the next character without advancing.
    fn peek(&mut self) -> Option<char> {
        self.data.peek().copied()
    }

    /// Consume and return the next character while keeping track of position
    /// and line number.
    fn next(&mut self) -> Option<char> {
        println!("line: {}, col: {}, c: {:?}", self.line, self.col, self.data.peek());
        match self.data.next() {
            Some('\n') => {
                self.col = 0;
                self.line += 1;
                Some('\n')
            }
            Some(c) => {
                self.col += 1;
                Some(c)
            }
            None => None
        }
    }

    /// Discard the next character.
    ///
    /// # Panics
    ///
    /// Will panic if there is no next character. This method should only be
    /// used after you have called [`peek()`] and know there is another
    /// character to read.
    fn skip(&mut self) {
        self.next().unwrap();
    }

    /// Untility method to create a new error object using the stored line
    /// and column values of the tokenizer.
    fn error(&self, kind: ErrorKind) -> Error {
        Error { line: self.line, col: self.col, kind }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    Tokenizer::new(input).tokenize()
}

/// Determine if a character can be used as an identifier for a variable of
/// method.
///
/// # Examples
/// ```
/// use clasp_parsing::tokenization::is_valid_identifier_char;
///
/// // All a-z and A-Z are allowed anywhere in the identifier.
/// assert!(is_valid_identifier_char('c', true));
/// assert!(is_valid_identifier_char('c', false));
/// // Numbers are not allowed as the first character.
/// assert!(is_valid_identifier_char('0', false));
/// assert!(!is_valid_identifier_char('0', true));
/// ```
pub fn is_valid_identifier_char(c: char, is_first_char: bool) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        '0'..='9' if !is_first_char => true,
        _ => false
    }
}
