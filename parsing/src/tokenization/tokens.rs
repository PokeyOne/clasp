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
    Caret
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

impl Symbol {
    pub fn functional_name(&self) -> String {
        match self {
            Symbol::Plus => "+".to_string(),
            Symbol::Minus => "-".to_string(),
            Symbol::Multiply => "*".to_string(),
            Symbol::Divide => "/".to_string(),
            Symbol::Equal => "=".to_string(),
            Symbol::NotEqual => "!=".to_string(),
            Symbol::LessThan => "<".to_string(),
            Symbol::GreaterThan => ">".to_string(),
            Symbol::LessThanOrEqual => "<=".to_string(),
            Symbol::GreaterThanOrEqual => ">=".to_string(),
            Symbol::And => "&&".to_string(),
            Symbol::Or => "||".to_string(),
            Symbol::Not => "!".to_string(),
            Symbol::Dot => ".".to_string(),
            Symbol::Comma => ",".to_string(),
            Symbol::Semicolon => ";".to_string(),
            Symbol::Colon => ":".to_string(),
            Symbol::QuestionMark => "?".to_string(),
            Symbol::Bar => "|".to_string(),
            Symbol::Ampersand => "&".to_string(),
            Symbol::Caret => "^".to_string()
        }
    }
}

impl Keyword {
    pub fn functional_name(&self) -> String {
        match self {
            Keyword::Fn => "fn".to_string()
        }
    }
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
