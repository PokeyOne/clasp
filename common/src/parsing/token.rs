pub enum BinOpToken {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    And,
    Or,
    ShiftLeft,
    ShiftRight
}

// TODO: For now only math related tokens are in here
pub enum TokenKind {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    BinOp(BinOpToken),
    Identifier,
}

pub struct Token {
    value: String,
    kind: TokenKind
}
