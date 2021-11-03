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
    content: Option<Box<Vec<ASTNode>>>,
    raw: Option<RawValue>
}

impl ASTNode {
    pub fn new(method_name: String, content: Vec<ASTNode>) -> ASTNode {
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
        assert_eq!(
            3,
            match node.raw.unwrap() {
                RawValue::Int(val) => *val,
                _ => panic!("node was not integer")
            }
        );
    }

    #[test]
    fn create_raw_integer_node() {
        let node = ASTNode::iraw(3);

        assert_eq!("raw", node.method_name);
        assert_eq!(true, node.content.is_none());
        assert_eq!(
            3,
            match node.raw.unwrap() {
                RawValue::Int(val) => *val,
                _ => panic!("node was not integer")
            }
        );
    }

    #[test]
    fn create_new() {
        let mut args: Vec<ASTNode> = Vec::new();
        args.push(ASTNode::iraw(3));
        args.push(ASTNode::iraw(5));

        let node = ASTNode::new("add".to_string(), args);

        assert_eq!("add".to_string(), node.method_name);
        assert_eq!(2, node.content.as_ref().unwrap().len());
        assert_eq!(true, node.raw.is_none());

        match node.content {
            Some(vec) => {
                assert_eq!(
                    3,
                    match vec[0].raw.as_ref().unwrap() {
                        RawValue::Int(val) => **val,
                        _ => panic!("Node was not integer")
                    }
                );
                assert_eq!(
                    5,
                    match vec[1].raw.as_ref().unwrap() {
                        RawValue::Int(val) => **val,
                        _ => panic!("Node was not integer")
                    }
                );
            }
            None => panic!("Node has no content")
        }
    }
}
