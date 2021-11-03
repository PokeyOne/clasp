pub mod ast;
pub mod string_tracker;

use ast::nodes::ASTNode;
use ast::AbstractSyntaxTree;
use string_tracker::StringTracker;

pub fn tokenize_text(input: String) -> AbstractSyntaxTree {
    let base_nodes: Vec<ASTNode> = process_node_list(StringTracker::new(input, 0));

    return AbstractSyntaxTree::new(ASTNode::iraw(5));
}

fn process_node_list(tracker: StringTracker) -> Vec<ASTNode> {
    return Vec::new();
}
