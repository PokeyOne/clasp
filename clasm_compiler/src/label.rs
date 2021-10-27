pub struct Label {
    name: String,
    location: u64
}

impl Label {
    pub fn new(name: String, location: u64) -> Label {
        Label { name: name, location: location }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_and_use_label() {
        let l: Label = Label::new("test_name".to_string(), 24);

        assert_eq!(l.name, "test_name");
        assert_eq!(l.location, 24);
    }
}
