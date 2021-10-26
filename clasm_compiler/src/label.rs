pub struct Label {
    location: u64,
    name: String
}

impl Label {
    pub fn new(location: u64, name: String) -> Label {
        Label { location: location, name: name }
    }
}

pub struct LabelCollection<'a> {
    head: Option<LabelCollectionNode<'a>>,
    count: u64
}

struct LabelCollectionNode<'a> {
    value: Label,
    right: Option<&'a LabelCollectionNode<'a>>,
    left: Option<&'a LabelCollectionNode<'a>>
}

impl<'a> LabelCollection<'a> {
    pub fn new() -> LabelCollection<'a> {
        LabelCollection { head: None, count: 0 }
    }

    pub fn insert_label(&mut self, label: Label) {
        match &mut self.head {
            None => {
                self.head = Some(LabelCollectionNode::new(label));
            },
            Some(ref mut head) => {
                head.insert_label(label);
            }
        }

        /*
        if self.head == None {
            // TODO: Label collection node :: new()
            self.head = Some(LabelCollectionNode { value: label, right: None, left: None })
        } else {
            self.head.unwrap().insert_label(label);
        }*/
        self.count += 1;
    }

    pub fn insert(&mut self, location: u64, name: String) {
        self.insert_label(Label::new(location, name));
    }
}

impl<'a> LabelCollectionNode<'a> {
    fn new(value: Label) -> LabelCollectionNode<'a> {
        LabelCollectionNode { value: value, right: None, left: None }
    }

    fn insert_label(&mut self, label: Label) {
        match self.value.name.cmp(&label.name) {
            Less => {
                match self.right {
                    None => {
                        self.right = Some(&LabelCollectionNode::new(label));
                    },
                    Some(right) => {
                        right.insert_label(label);
                    }
                }
            },
            Greater => {
                match self.left {
                    None => {
                        let tmp: &'a LabelCollectionNode = &LabelCollectionNode::new(label);
                        self.left = Some(tmp);
                    },
                    Some(left) => {
                        left.insert_label(label);
                    }
                }
            },
            Equal => {
                self.value = label;
            }
        }
    }
}
