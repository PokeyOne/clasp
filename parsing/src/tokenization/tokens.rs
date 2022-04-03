/// The base type of all tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Identifier(String)
}
