#[derive(Debug)]
pub enum RawValue {
    Int(Box<i64>),
    String(Box<String>),
    UnsignedInt(Box<u64>),
    Pointer(Box<u64>)
}

#[derive(Debug)]
pub struct ASTNode {
    method_name: String,
    content: Option<Box<ASTNode>>,
    raw: Option<RawValue>,
}

impl ASTNode {
    pub fn new(method_name: String, content: ASTNode) -> ASTNode {
        ASTNode {
            method_name: method_name,
            content: Some(Box::new(content)),
            raw: None
        }
    }

    pub fn raw(value: RawValue) -> ASTNode {
        ASTNode {
            method_name: "raw".to_string(),
            content: None,
            raw: Some(value)
        }
    }

    pub fn iraw(value: i64) -> ASTNode {
        ASTNode::raw(RawValue::Int(Box::new(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_raw_node() {
        let node = ASTNode::raw(RawValue::Int(Box::new(3)));

        assert_eq!("raw", node.method_name);
        assert_eq!(true, node.content.is_none());
        assert_eq!(3, match node.raw.unwrap() { RawValue::Int(val) => *val, _ => panic!("node was not integer") });
    }

    #[test]
    fn create_raw_integer_node() {
        let node = ASTNode::iraw(3);

        assert_eq!("raw", node.method_name);
        assert_eq!(true, node.content.is_none());
        assert_eq!(3, match node.raw.unwrap() { RawValue::Int(val) => *val, _ => panic!("node was not integer") });
    }

    #[test]
    fn create_new() {
        // TODO: Make content a vector. Easier for arguments and methods and such
    }
}
