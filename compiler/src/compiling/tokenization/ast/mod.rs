mod nodes;

use nodes::ASTNode;

pub struct AbstractSyntaxTree {
    head: Option<Box<ASTNode>>
}

impl AbstractSyntaxTree {

    // TODO: This need severe work
    pub fn new() -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            head: None
        }
    }
}
