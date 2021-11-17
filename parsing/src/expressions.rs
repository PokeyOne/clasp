pub mod conditional;

use conditional::ConditionalExpression;

pub enum Expression {
    Conditional(ConditionalExpression)
}
