pub enum Token {
    Whitespace(String), // Things like newlines and spaces
    Comment(String), // The full comment
    Identifier(String), // variabls, functions, types, etc
    StringLiteral(String),
    NumberLiteral(String),
    OpeningBracket, // (
    ClosingBracket, // )
    OpeningSquareBracket, // [
    ClosingSquareBracket, // ]
    KeyStarter, // '
}