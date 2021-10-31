pub mod ast;

use ast::AbstractSyntaxTree;
use ast::nodes::ASTNode;

pub fn tokenize_text(input: String) -> AbstractSyntaxTree {
    return AbstractSyntaxTree::new(ASTNode::iraw(5));
}
