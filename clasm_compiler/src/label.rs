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

#[derive(Debug)]
pub struct LabelCollection<'a> {
    head: Option<Box<LabelNode<'a>>>,
    label_nodes: Vec<LabelNode<'a>>
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

    pub fn size(&self) -> u64 {
        1 + self.left_size() + self.right_size()
    }

    fn left_size(&self) -> u64 {
        match &self.l {
            None => 0,
            Some(val) => val.size()
        }
    }

    fn right_size(&self) -> u64 {
        match &self.r {
            None => 0,
            Some(val) => val.size()
        }
    }
}

impl<'a> LabelCollection<'a> {
    pub fn new() -> LabelCollection<'a> {
        LabelCollection { head: None, label_nodes: Vec::new() }
    }

    pub fn size(&self) -> u64 {
        match &self.head {
            None => 0,
            Some(val) => val.size()
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false
        }
    }

    pub fn insert(&mut self, name: String, location: u64) {
        match &self.head {
            None => {
                self.head = Some(Box::new(LabelNode::new(Label::new(name, location))));
            }, _ => panic!("balh")
        }
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
            Ordering::Equal => true,
            _ => false
        });

        assert_eq!(true, match a.cmp(&c) { Ordering::Greater => true, _ => false });
    }

    #[test]
    fn size_of_node() {
        let la: Label = Label::new("a".to_string(), 24);
        let lb: Label = Label::new("b".to_string(), 48);
        let lc: Label = Label::new("c".to_string(), 72);

        let mut lna: LabelNode = LabelNode::new(la);
        let mut lnb: LabelNode = LabelNode::new(lb);
        let mut lnc: LabelNode = LabelNode::new(lc);

        assert_eq!(lna.size(), 1);
        assert_eq!(lnb.size(), 1);
        assert_eq!(lnc.size(), 1);

        lnc.l = Some(&mut lnb);
        assert_eq!(lnc.size(), 2);
        lna.r = Some(&mut lnc);
        assert_eq!(lna.size(), 3);
    }

    #[test]
    fn create_empty_collection() {
        let c: LabelCollection = LabelCollection::new();

        assert_eq!(true, c.is_empty());
        assert_eq!(0, c.size());
    }

    #[test]
    fn insert_one_element_to_collection() {
        let mut c = LabelCollection::new();

        c.insert("blah".to_string(), 24);

        assert_eq!(false, c.is_empty());
        assert_eq!(1, c.size());
    }
}
