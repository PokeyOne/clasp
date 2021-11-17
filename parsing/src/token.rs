#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
pub enum LiteralKind {
    Identifier,
    Bool,
    String,
    Integer(u16), // argument is size
    UnsignedInteger(u16),
    Double(u16)
}

#[derive(PartialEq, Clone)]
pub enum BracketDirection {
    Opening,
    Closing
}

#[derive(PartialEq, Clone)]
pub enum BracketKind {
    Round(BracketDirection),
    Square(BracketDirection),
    Squigly(BracketDirection)
}

#[derive(PartialEq, Clone)]
pub enum TokenKind {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    BinOp(BinOpToken),
    Literal(LiteralKind),
    Bracket(BracketKind)
}

#[derive(PartialEq, Clone)]
pub struct Token {
    value: String,
    kind: TokenKind
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Self {
        Token { kind: kind, value: value }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}
