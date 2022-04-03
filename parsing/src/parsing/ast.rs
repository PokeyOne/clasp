#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub root: Expression
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Series(Vec<Expression>),
    Ident(String),
    Literal(Literal)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    StringLiteral(String)
}
