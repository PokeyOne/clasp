/// The base type of all tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(Literal)
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
