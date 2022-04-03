// mod std;

use std::collections::HashMap;
use clasp_parsing::parsing::ast::{Expression, Literal};
use crate::script::RuntimeException;

pub struct Context {
    variables: HashMap<String, Expression>,
    functions: HashMap<String, Function>,
}

impl Context {
    pub fn new() -> Self {
        let mut result = Context {
            variables: HashMap::new(),
            functions: HashMap::new(),
        };

        result.add_function("println", Function::new(|args: &[Expression]| {
            if args.len() != 1 {
                return Err(RuntimeException::WrongNumberOfArguments(1, args.len(), "println".to_string()));
            }
            match &args[0] {
                Expression::Literal(Literal::StringLiteral(s)) => println!("{}", s),
                ex => match super::execute_expression(ex, &mut Context::new()) {
                    Ok(Expression::Literal(Literal::StringLiteral(s))) => println!("{}", s),
                    _ => return Err(RuntimeException::ArgumentTypeMismatch {
                        function: "println".to_string(),
                        expected_type: "string".to_string(),
                        actual_type: ex.type_name(),
                        argument_index: 0,
                    }),
                },
            }
            Ok(args[0].clone())
        }));

        result
    }

    pub fn add_variable(&mut self, name: &str, value: Expression) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn add_function(&mut self, name: &str, function: Function) {
        self.functions.insert(name.to_string(), function);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Expression> {
        self.variables.get(name)
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}

pub enum Function {
    Rust(fn(&[Expression]) -> Result<Expression, RuntimeException>),
    // TODO: Allow for clasp-defined functions
}

impl Function {
    pub fn new(f: fn(&[Expression]) -> Result<Expression, RuntimeException>) -> Self {
        Function::Rust(f)
    }

    pub fn call(&self, args: &[Expression]) -> Result<Expression, RuntimeException> {
        match self {
            Function::Rust(f) => f(args),
        }
    }
}
