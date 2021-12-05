use crate::tokenization::{Token, Literal as TLiteral};

pub struct Ast {
    expressions: Vec<Expression>,
}

pub enum Expression {
    Statement(Statement),
    Literal(Literal)
}

pub struct Statement {
    identifier: String,
    expressions: Vec<Expression>
}

pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool)
}

impl Ast {
    pub fn new(expressions: Vec<Expression>) -> Ast {
        Ast { expressions }
    }

    pub fn add_expression(&mut self, expression: Expression) {
        self.expressions.push(expression);
    }

    pub fn get_expressions(&self) -> &Vec<Expression> {
        &self.expressions
    }
}

impl Literal {
    pub fn from_literal_token(token: Token) -> Option<Literal> {
        match token {
            Token::Literal(TLiteral::Number(value)) => Some(Literal::Number(value)),
            Token::Literal(TLiteral::String(value)) => Some(Literal::String(value)),
            Token::Literal(TLiteral::Boolean(value)) => Some(Literal::Boolean(value)),
            _ => None
        }
    }
}