mod context;

use context::Context;
use clasp_parsing::parsing::ast::{Ast, Expression};

pub struct Script {
    pub ast: Ast,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeException {
    UndefinedVariableOrFunction(String),
    /// The wrong number of arguments were passed to a function.
    ///
    /// # Tuple Elements
    /// * `.0`: The number of arguments that were expected.
    /// * `.1`: The number of arguments that were passed.
    /// * `.2`: The name of the function that was called.
    WrongNumberOfArguments(usize, usize, String),
    FeatureNotImplemented(String)
}

impl Script {
    pub fn from_ast(ast: Ast) -> Script {
        Script {
            ast
        }
    }

    pub fn execute(&self) -> Result<(), RuntimeException> {
        let mut context = Context::new();

        execute_expression(&self.ast.root, &mut context)?;

        Ok(())
    }
}

fn execute_expression(expr: &Expression, context: &mut Context) -> Result<Expression, RuntimeException> {
    match expr {
        Expression::Series(series) => {
            if series.is_empty() {
                return Ok(Expression::Series(series.clone()));
            }

            if series.len() == 1 {
                return execute_expression(&series[0], context);
            }

            match &series[0] {
                Expression::Literal(_) | Expression::Series(_) => {
                    // Not a function call, so just execute the series.
                    let new_series = series.clone()
                        .into_iter()
                        .map(|expr| execute_expression(&expr, context))
                        .collect::<Result<Vec<Expression>, RuntimeException>>()?;
                    Ok(Expression::Series(new_series))
                }
                Expression::Ident(ident) => {
                    match context.get_function(ident) {
                        Some(func) => {
                            func.call(&series[1..])
                        }
                        None => match context.get_variable(ident) {
                            Some(val) => {
                                let new_series = series.clone()
                                    .into_iter()
                                    .map(|expr| execute_expression(&expr, context))
                                    .collect::<Result<Vec<Expression>, RuntimeException>>()?;
                                Ok(Expression::Series(new_series))
                            }
                            None => Err(RuntimeException::UndefinedVariableOrFunction(ident.clone()))
                        }
                    }
                }
            }
        },
        Expression::Ident(ident) => {
            match context.get_function(ident) {
                Some(func) => {
                    func.call(&[])
                }
                None => match context.get_variable(ident) {
                    Some(val) => Ok(val.clone()),
                    None => Err(RuntimeException::UndefinedVariableOrFunction(ident.clone())),
                }
            }
        },
        Expression::Literal(literal) => Ok(Expression::Literal(literal.clone()))
    }
}
