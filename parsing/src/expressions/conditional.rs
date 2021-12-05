#[cfg(test)]
mod tests;

use crate::token::{BracketKind, BracketDirection, LiteralKind, Token, TokenKind, BinOpToken};

#[derive(Debug)]
pub struct ConditionalExpression {
    value: ConditionalNode
}

impl ConditionalExpression {
    fn new(value: ConditionalNode) -> Self {
        ConditionalExpression { value }
    }
}

#[derive(Debug)]
pub enum ConditionalNode {
    Literal(ConditionalLiteral),
    Operation(ConditionalOperation),
}

#[derive(PartialEq, Clone, Debug)]
pub enum ConditionalLiteral {
    Bool(bool),
    Identifier(Token)
}

#[derive(PartialEq, Clone, Debug)]
pub enum ConditionalOperationKind {
    And,
    Or,
    Eq,
    Gt,
    Ge,
    Lt,
    Le
}
type Cok = ConditionalOperationKind;

#[derive(Debug)]
pub struct ConditionalOperation {
    kind: ConditionalOperationKind,
    left: Box<ConditionalNode>,
    right: Box<ConditionalNode>
}

impl ConditionalOperation {
    fn new(kind: Cok, left: ConditionalNode, right: ConditionalNode) -> Self {
        Self { kind, left: Box::new(left), right: Box::new(right) }
    }
}

/// This is just an individual element that has no idea about context or trees.
/// Meant for parsing from array.
#[derive(PartialEq, Clone, Debug)]
pub enum ConditionalToken {
    Operator(ConditionalOperationKind),
    Bracket(BracketDirection),
    Literal(ConditionalLiteral)
}

fn map_tokens(tokens: &Vec<Token>) -> Result<Vec<ConditionalToken>, &'static str> {
    let mut result: Vec<ConditionalToken> = Vec::new();

    for t in tokens {
        let ct = match t.kind() {
            TokenKind::Lt => ConditionalToken::Operator(Cok::Lt),
            TokenKind::Le => ConditionalToken::Operator(Cok::Le),
            TokenKind::Gt => ConditionalToken::Operator(Cok::Gt),
            TokenKind::Ge => ConditionalToken::Operator(Cok::Ge),
            TokenKind::Eq => ConditionalToken::Operator(Cok::Eq),
            TokenKind::Literal(lit) => {
                match lit {
                    LiteralKind::Identifier => ConditionalToken::Literal(ConditionalLiteral::Identifier(t.clone())),
                    LiteralKind::Bool => match t.value().as_str() {
                        "true" => ConditionalToken::Literal(ConditionalLiteral::Bool(true)),
                        "false" => ConditionalToken::Literal(ConditionalLiteral::Bool(false)),
                        _ => return Err("Unknown boolean type")
                    }
                    _ => return Err("Unsupported literal type")
                }
            },
            TokenKind::Bracket(bk) => match bk {
                BracketKind::Round(dir) => ConditionalToken::Bracket(dir.clone()),
                _ => return Err("Only round brackets allowed in conditional")
            },
            TokenKind::BinOp(bin_op_t) => match bin_op_t {
                BinOpToken::And => ConditionalToken::Operator(Cok::And),
                BinOpToken::Or => ConditionalToken::Operator(Cok::Or),
                _ => return Err("Unsupported bin op type")
            }
            _ => return Err("Unsupported token in conditional expression")
        };

        result.push(ct);
    }

    Ok(result)
}

pub trait Shuntable {
    fn shunting_yard(&self) -> Self;
}

impl Shuntable for Vec<ConditionalToken> {
    fn shunting_yard(&self) -> Self {
        let mut queue: Vec<ConditionalToken> = self.clone();

        let mut output_stack: Vec<ConditionalToken> = Vec::new();
        let mut operator_stack: Vec<ConditionalToken> = Vec::new();

        // TODO: There should be some sort of precedence: "and"/"or" should be
        //       higher than gt, lt, eq, le, ge
        while let Some(current) = queue.pop() {
            use ConditionalToken::*;

            match current {
                // TODO: I think for op precedence can just pop from operator
                //       stack while the top of the stack is higher/lower precedence.
                Operator(_) => operator_stack.push(current),
                Literal(_) => output_stack.push(current),
                Bracket(ref dir) => match dir {
                    BracketDirection::Opening => operator_stack.push(current),
                    BracketDirection::Closing => {
                        while let Some(op) = operator_stack.pop() {
                            if op == ConditionalToken::Bracket(BracketDirection::Opening) {
                                break;
                            } else {
                                output_stack.push(op);
                            }
                        }
                    }
                }
            }
        }

        while let Some(op) = operator_stack.pop() {
            output_stack.push(op);
        }

        output_stack
    }
}

fn construct_node(tokens: &mut Vec<ConditionalToken>) -> Option<ConditionalNode> {
    use ConditionalToken as Ct;

    match tokens.pop() {
        None => None,
        Some(t) => match t {
            Ct::Operator(k) => {
                let left = construct_node(tokens)?;
                let right = construct_node(tokens)?;

                let cop = ConditionalOperation::new(k, left, right);

                Some(ConditionalNode::Operation(cop))
            },
            Ct::Literal(lit) => match lit {
                ConditionalLiteral::Bool(_) => Some(ConditionalNode::Literal(lit)),
                _ => panic!("not implemented")
            },
            Ct::Bracket(_k) => panic!("There should not be brackets")
        }
    }
}

impl ConditionalExpression {
    pub fn from_tokens(tokens: &Vec<Token>) -> Result<Self, &'static str> {
        let mut processed_tokens = map_tokens(tokens)?.shunting_yard();

        match construct_node(&mut processed_tokens) {
            Some(val) => Ok(ConditionalExpression::new(val)),
            None => Err("could not construct node")
        }
    }
}
