use std::fmt;

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub enum LiteralKind {
    Identifier,
    Bool,
    String,
    Integer(u16), // argument is size
    UnsignedInteger(u16),
    Double(u16)
}

#[derive(PartialEq, Clone, Debug)]
pub enum BracketDirection {
    Opening,
    Closing
}

#[derive(PartialEq, Clone, Debug)]
pub enum BracketKind {
    Round(BracketDirection),
    Square(BracketDirection),
    Squigly(BracketDirection)
}

#[derive(PartialEq, Clone, Debug)]
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

impl TokenKind {
    pub fn try_name(&self) -> Option<String> {
        use TokenKind::*;

        match &self {
            Lt => Some("<".to_string()),
            Gt => Some(">".to_string()),
            Le => Some("<=".to_string()),
            Ge => Some("<=".to_string()),
            Eq => Some("=".to_string()),
            _ => None
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Token {
    value: String,
    kind: TokenKind
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Token")
            .field(&self.kind)
            .field(&self.value)
            .finish()
    }
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Self {
        Token { kind, value }
    }

    pub fn try_from_kind(kind: TokenKind) -> Option<Self> {
        match kind.try_name() {
            Some(name) => Some(Self::new(kind, name)),
            None => None
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}
