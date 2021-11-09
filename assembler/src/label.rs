use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
    pub location: u64
}

#[derive(Debug)]
struct LabelNode {
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
    fn new(label: Label) -> LabelNode {
        LabelNode {
            label: label,
            l: None,
            r: None
        }
    }

    fn size(&self) -> u64 {
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
