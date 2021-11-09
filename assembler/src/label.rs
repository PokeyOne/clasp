use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Label {
    name: String,
    location: u64
}

#[derive(Debug)]
pub struct LabelNode {
    label: Label,
    l: Option<Box<LabelNode>>,
    r: Option<Box<LabelNode>>
}

#[derive(Debug)]
pub struct LabelCollection {
    head: Option<Box<LabelNode>>
}

impl Label {
    pub fn new(name: String, location: u64) -> Label {
        Label {
            name: name,
            location: location
        }
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

impl Eq for Label {}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl LabelNode {
    pub fn new(label: Label) -> LabelNode {
        LabelNode {
            label: label,
            l: None,
            r: None
        }
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

    fn print_ordered_list(&self) {
        match &self.r {
            None => {}
            Some(val) => val.print_ordered_list()
        };

        println!("{} -> {}", &self.label.name, &self.label.location);

        match &self.l {
            None => {}
            Some(val) => val.print_ordered_list()
        };
    }
}

impl LabelCollection {
    pub fn new() -> LabelCollection {
        LabelCollection { head: None }
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

    // This was hell with lifetimes and references and written on like a month
    // of Rust experience. Seems to pass test, but definitely review with more
    // experience
    pub fn insert(&mut self, name: String, location: u64) {
        match &mut self.head {
            None => {
                self.head = Some(Box::new(LabelNode::new(Label::new(name, location))));
            }
            Some(head_node) => {
                let mut current_node: &mut LabelNode = &mut *head_node;

                loop {
                    let label_copy: Label = current_node.label.clone();

                    match label_copy.cmp(&Label::new(name.clone(), 0)) {
                        Ordering::Equal => {
                            current_node.label.location = location;
                            break;
                        }
                        Ordering::Greater => {
                            if current_node.r.is_none() {
                                current_node.r =
                                    Some(Box::new(LabelNode::new(Label::new(name, location))));
                                break;
                            }

                            current_node = &mut *current_node.r.as_mut().unwrap();
                        }
                        Ordering::Less => {
                            if current_node.l.is_none() {
                                current_node.l =
                                    Some(Box::new(LabelNode::new(Label::new(name, location))));
                                break;
                            }

                            current_node = &mut *current_node.l.as_mut().unwrap();
                        }
                    };
                }
            }
        }
    }

    /// Retrieve the location of a label. None if not in tree.
    pub fn retrieve(&self, name: String) -> Option<u64> {
        match &self.head {
            None => return None,
            Some(head_node) => {
                let mut current_node: &LabelNode = head_node;

                loop {
                    let label_copy: Label = current_node.label.clone();

                    match label_copy.cmp(&Label::new(name.clone(), 0)) {
                        Ordering::Equal => return Some(current_node.label.location),
                        Ordering::Greater => {
                            if current_node.r.is_none() {
                                return None;
                            }

                            current_node = current_node.r.as_ref().unwrap();
                        }
                        Ordering::Less => {
                            if current_node.l.is_none() {
                                return None;
                            }

                            current_node = current_node.l.as_ref().unwrap();
                        }
                    }
                }
            }
        };
    }

    // TODO: Implement interator trait and then this is trivial. This method
    //       is a quick implementation for debug purposes.
    pub fn print_ordered_list(&self) {
        match &self.head {
            None => {}
            Some(head_node) => head_node.print_ordered_list()
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

        assert_eq!(
            true,
            match ln.l {
                Some(_) => false,
                None => true
            }
        );
        assert_eq!(
            true,
            match ln.r {
                Some(_) => false,
                None => true
            }
        );
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

        assert_eq!(
            true,
            match a.cmp(&b) {
                Ordering::Equal => true,
                _ => false
            }
        );

        assert_eq!(
            true,
            match a.cmp(&c) {
                Ordering::Greater => true,
                _ => false
            }
        );
    }

    #[test]
    fn size_of_node() {
        let la: Label = Label::new("a".to_string(), 24);
        let lb: Label = Label::new("b".to_string(), 48);
        let lc: Label = Label::new("c".to_string(), 72);

        let mut lna: LabelNode = LabelNode::new(la);
        let lnb: LabelNode = LabelNode::new(lb);
        let mut lnc: LabelNode = LabelNode::new(lc);

        assert_eq!(lna.size(), 1);
        assert_eq!(lnb.size(), 1);
        assert_eq!(lnc.size(), 1);

        lnc.l = Some(Box::new(lnb));
        assert_eq!(lnc.size(), 2);
        lna.r = Some(Box::new(lnc));
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

        c.insert("blah".to_string(), 48);

        assert_eq!(false, c.is_empty());
        assert_eq!(1, c.size());
    }

    #[test]
    fn insert_several_elements_to_label_collection() {
        let mut c = LabelCollection::new();

        c.insert("blah".to_string(), 24);
        c.insert("apples".to_string(), 48);

        assert_eq!(false, c.is_empty());
        assert_eq!(2, c.size());

        c.insert("bananas".to_string(), 32);
        c.insert("baa".to_string(), 64);

        assert_eq!(4, c.size());
    }

    #[test]
    fn retrieve_from_empty_label_collection() {
        let c = LabelCollection::new();

        assert_eq!(true, c.retrieve("some".to_string()).is_none());
    }

    #[test]
    fn retrieve_from_single_celled_label_collection() {
        let mut c = LabelCollection::new();

        c.insert("_main".to_string(), 540);

        assert_eq!(Some(540u64), c.retrieve("_main".to_string()));
        assert_eq!(true, c.retrieve("_other".to_string()).is_none());
    }

    #[test]
    fn full_test_of_label_collection() {
        let mut c = LabelCollection::new();

        assert_eq!(true, c.is_empty());
        assert_eq!(0, c.size());

        c.insert("banana".to_string(), 64);
        c.insert("apple".to_string(), 8);
        c.insert("pear".to_string(), 48);
        assert_eq!(3, c.size());
        c.insert("starfruit".to_string(), 40);
        c.insert("carrot".to_string(), 80);

        assert_eq!(5, c.size());

        assert_eq!(true, c.retrieve("non-existent".to_string()).is_none());
        assert_eq!(Some(64), c.retrieve("banana".to_string()));
        assert_eq!(Some(8), c.retrieve("apple".to_string()));
        assert_eq!(Some(48), c.retrieve("pear".to_string()));
        assert_eq!(Some(40), c.retrieve("starfruit".to_string()));
        assert_eq!(Some(80), c.retrieve("carrot".to_string()));

        c.insert("starfruit".to_string(), 4000);

        assert_eq!(5, c.size());
        assert_eq!(Some(4000), c.retrieve("starfruit".to_string()));
    }
}