pub mod nodes;

use nodes::ASTNode;

pub struct AbstractSyntaxTree {
    head: Option<Box<ASTNode>>
}

impl AbstractSyntaxTree {

    pub fn new(head: ASTNode) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            head: Some(Box::new(head))
        }
    }
}
