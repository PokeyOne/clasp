use crate::token::Token;

pub enum Operator {
    Comparison(ComparisonOperator)
    // TODO: math operator
}

pub enum ComparisonOperator {
    Lt,
    Le,
    Gt,
    Ge,
    Eq
}

impl ComparisonOperator {
    from_token(t: &Token) -> Option<ComparisonOperator> {
        use ComparisonOperator::*;

        match t.kind {
            TokenKind::Lt => Some(Lt),
            TokenKind::Le => Some(Le),
            TokenKind::Gt => Some(Gt),
            TokenKind::Ge => Some(Ge),
            TokenKind::Eq => Some(Eq),
            _ => None
        }
    }
}
