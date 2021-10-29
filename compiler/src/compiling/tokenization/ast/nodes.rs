pub struct ASTNode {
    method_name: String,
    content: Option<Box<ASTNode>>,
    raw: Option<String>
}

impl ASTNode {
    pub fn new(method_name: String, content: ASTNode) -> ASTNode {
        ASTNode {
            method_name: method_name,
            content: Some(Box::new(content)),
            raw: None
        }
    }

    pub fn raw(value: String) -> ASTNode {
        ASTNode {
            method_name: "raw".to_string(),
            content: None,
            raw: Some(value)
        }
    }
}
