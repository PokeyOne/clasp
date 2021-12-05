/// The base type of all tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(Literal),
    OpenBracket,
    CloseBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Fn
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    Not,
    Dot,
    Comma,
    Semicolon,
    Colon,
    QuestionMark,
    Bar,
    Ampersand,
}

/// The base type of all literals.
///
/// The stored value in a string literal is the raw string, without the quotes.
/// This includes multi-word strings. All escape characters are processed.
///
/// The stored value in the number literal is the interpreted number. Note
/// that all numbers are stored as `f64`s intentionally. The language is meant
/// to be used quickly and does not care about number types. In the future this
/// may change.
///
/// The stored value in the boolean literal is the interpreted boolean.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool)
}

#[macro_export]
macro_rules! string_literal {
    ($s:expr) => {
        Token::Literal(Literal::String($s.to_string()))
    };
}

#[macro_export]
macro_rules! number_literal {
    ($n:expr) => {
        Token::Literal(Literal::Number($n as f64))
    };
}

#[macro_export]
macro_rules! boolean_literal {
    ($b:expr) => {
        Token::Literal(Literal::Boolean($b))
    };
}
