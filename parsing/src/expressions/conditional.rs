use crate::token::{BracketKind, BracketDirection, LiteralKind, Token, TokenKind};

pub struct ConditionalExpression {
    value: ConditionalNode
}

pub enum ConditionalNode {
    Literal(ConditionalLiteral),
    Operation(ConditionalOperation),
}

#[derive(PartialEq, Clone)]
pub enum ConditionalLiteral {
    Bool(bool),
    Identifier(Token)
}

#[derive(PartialEq, Clone)]
pub enum ConditionalOperationKind {
    Eq,
    Gt,
    Ge,
    Lt,
    Le
}
type Cok = ConditionalOperationKind;

pub struct ConditionalOperation {
    kind: ConditionalOperationKind,
    left: Box<ConditionalNode>,
    right: Box<ConditionalNode>
}

/// This is just an individual element that has no idea about context or trees.
/// Meant for parsing from array.
#[derive(PartialEq, Clone)]
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
            }
            _ => return Err("Unsupported token in conditional expression")
        };

        result.push(ct);
    }

    return Ok(result);
}

pub trait Shuntable {
    fn shunting_yard(&self) -> Self;
}

impl Shuntable for Vec<ConditionalToken> {
    fn shunting_yard(&self) -> Self {
        let mut queue: Vec<ConditionalToken> = self.clone();

        let mut output_stack: Vec<ConditionalToken> = Vec::new();
        let mut operator_stack: Vec<ConditionalToken> = Vec::new();

        while let Some(current) = queue.pop() {
            use ConditionalToken::*;

            match current {
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

        return output_stack;
    }
}

/* This was an idea
struct ConditionalNodeFactory {
    literal: Option<ConditionalLiteral>,
    operation: Option<ConditionalOperation>,
    operation_kind: Option<ConditionalOperationKind>,
    left: Option<Box<ConditionalOperation>>,
    right: Option<Box<ConditionalOperation>>
}

impl ConditionalNodeFactory {
    // TODO
}*/

// TODO: Recursive function
fn construct_node(tokens: &Vec<ConditionalToken>) -> Option<ConditionalNode> {
    use ConditionalToken as Ct;

    match tokens.pop() {
        None => None,
        Some(t) => match t {
            Ct::Operator,
            Ct::Bracket,
            Ct::Literal
        }
    }
}

impl ConditionalExpression {
    pub fn from_tokens(tokens: &Vec<Token>) -> Result<Self, &'static str> {
        let mut processed_tokens = map_tokens(tokens)?.shunting_yard();

        return Err("not implemented");
    }
}
