#[derive(Debug)]
pub enum ConditionalType {
    Equal,
    Greater,
    Lesser,
    GreaterEqual,
    LesserEqual
}

impl ConditionalType {
    pub fn perform(&self, a: u64, b: u64) -> bool {
        println!("performing: {:?}", &self);

        match &self {
            ConditionalType::Equal => a == b,
            ConditionalType::Greater => a > b,
            ConditionalType::GreaterEqual => a >= b,
            ConditionalType::Lesser => a < b,
            ConditionalType::LesserEqual => a <= b
        }
    }
}

// Tests are put within the file because it's so small
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal() {
        let ct = ConditionalType::Equal;

        assert!(ct.perform(4, 4));
        assert!(ct.perform(0, 0));
        assert!(!ct.perform(6, 92));
    }

    #[test]
    fn greater() {
        let ct = ConditionalType::Greater;

        assert!(ct.perform(5, 4));
        assert!(ct.perform(63041, 0));
        assert!(!ct.perform(6, 92));
        assert!(!ct.perform(92, 92));
    }

    #[test]
    fn greater_or_equal() {
        let ct = ConditionalType::GreaterEqual;

        assert!(ct.perform(5, 4));
        assert!(ct.perform(63041, 0));
        assert!(!ct.perform(6, 92));
        assert!(ct.perform(92, 92));
    }

    #[test]
    fn lesser() {
        let ct = ConditionalType::Lesser;

        assert!(ct.perform(4, 5));
        assert!(ct.perform(0, 63041));
        assert!(!ct.perform(92, 6));
        assert!(!ct.perform(92, 92));
    }

    #[test]
    fn lesser_or_equal() {
        let ct = ConditionalType::LesserEqual;

        assert!(ct.perform(4, 5));
        assert!(ct.perform(0, 63041));
        assert!(!ct.perform(92, 6));
        assert!(ct.perform(92, 92));
    }
}
