use std::cmp::Ordering;

#[derive(Debug)]
pub struct Label {
    name: String,
    location: u64
}

#[derive(Debug)]
pub struct LabelNode<'a> {
    label: Label,
    l: Option<&'a mut LabelNode<'a>>,
    r: Option<&'a mut LabelNode<'a>>
}

impl Label {
    pub fn new(name: String, location: u64) -> Label {
        Label { name: name, location: location }
    }
}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Label { }

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl<'a> LabelNode<'a> {
    pub fn new(label: Label) -> LabelNode<'a> {
        LabelNode { label: label, l: None, r: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_use_label() {
        let l: Label = Label::new("test_name".to_string(), 24);

        assert_eq!(l.name, "test_name");
        assert_eq!(l.location, 24);
    }

    #[test]
    fn create_and_read_label_node() {
        let l: Label = Label::new("test_name".to_string(), 24);
        let ln: LabelNode = LabelNode::new(l);

        assert_eq!(true, match ln.l { Some(_) => false, None => true });
        assert_eq!(true, match ln.r { Some(_) => false, None => true });
        assert_eq!(ln.label.name, "test_name");
        assert_eq!(ln.label.location, 24);
    }

    #[test]
    fn label_comparisons() {
        let a: Label = Label::new("test_name".to_string(), 24);
        let b: Label = Label::new("test_name".to_string(), 48);
        let c: Label = Label::new("other_test_name".to_string(), 48);

        assert_eq!(true, a.eq(&b));
        assert_eq!(true, b.eq(&a));
        assert_eq!(false, a.eq(&c));
        assert_eq!(false, b.eq(&c));

        assert_eq!(true, match a.cmp(&b) {
            Eq => true,
            _ => false
        });

        assert_eq!(true, match a.cmp(&c) { Greater => true, _ => false });
    }
}
