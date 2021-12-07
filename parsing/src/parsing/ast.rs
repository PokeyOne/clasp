use crate::tokenization::{Token, Literal as TLiteral};

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    expressions: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Statement(Statement),
    Literal(Literal),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    identifier: String,
    expressions: Vec<Expression>
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn into_expressions(self) -> Vec<Expression> {
        self.expressions
    }

    pub fn reconstruct_code(&self) -> String {
        let mut code = String::new();
        for expression in &self.expressions {
            code.push_str(&expression.reconstruct_code());
        }
        code
    }
}

impl Expression {
    pub fn reconstruct_code(&self) -> String {
        match self {
            Expression::Statement(statement) => statement.reconstruct_code(),
            Expression::Literal(literal) => literal.reconstruct_code(),
            Expression::Identifier(identifier) => identifier.clone()
        }
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

    pub fn reconstruct_code(&self) -> String {
        match self {
            Literal::Number(value) => value.to_string(),
            Literal::String(value) => format!("\"{}\"", value),
            Literal::Boolean(value) => if *value {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
    }
}

impl Statement {
    pub fn new(identifier: String, expressions: Vec<Expression>) -> Statement {
        Statement { identifier, expressions }
    }

    pub fn get_identifier(&self) -> &String {
        &self.identifier
    }

    pub fn get_expressions(&self) -> &Vec<Expression> {
        &self.expressions
    }

    pub fn reconstruct_code(&self) -> String {
        let mut code = String::new();
        code.push('(');
        code.push_str(&self.identifier);
        for expression in &self.expressions {
            code.push(' ');
            code.push_str(&expression.reconstruct_code());
        }
        code.push(')');
        code
    }
}