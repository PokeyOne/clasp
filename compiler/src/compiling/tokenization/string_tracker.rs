pub struct StringTracker {
    data: String,
    index: usize
}

// Haha, why not just use a fucking iterator
impl StringTracker {
    pub fn new(data: String, index: usize) -> StringTracker {
        StringTracker {
            data: data,
            index: index
        }
    }

    pub fn increment(&mut self) {
        self.fwd(1);
    }

    pub fn fwd(&mut self, amount: usize) {
        self.index += amount;
    }

    pub fn chars_left(&self) -> usize {
        self.data.len() - self.index
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_space(&self) -> bool {
        match self.data.chars().nth(self.index) {
            Some(val) => val.is_whitespace(),
            None => false
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.data.chars().nth(self.index)
    }

    pub fn pop(&mut self) -> Option<char> {
        match self.peek() {
            Some(val) => {
                self.increment();
                Some(val)
            }
            None => None
        }
    }

    pub fn read(&mut self) -> Option<char> {
        self.pop()
    }

    pub fn skip_whitespace(&mut self) {
        while self.is_space() {
            self.increment();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_string_tracker() {
        let tracker = StringTracker::new("bogus".to_string(), 0);
        assert_eq!(5, tracker.chars_left());
        assert_eq!(5, tracker.len());

        let tracker = StringTracker::new("bogus".to_string(), 1);
        assert_eq!(4, tracker.chars_left());
        assert_eq!(5, tracker.len());
    }

    #[test]
    fn go_forward_in_data() {
        let mut tracker = StringTracker::new("b ogus".to_string(), 0);

        tracker.increment();
        assert_eq!(6, tracker.len());
        assert_eq!(5, tracker.chars_left());
        assert_eq!(true, tracker.is_space());

        tracker.fwd(2);
        assert_eq!(6, tracker.len());
        assert_eq!(3, tracker.chars_left());
        assert_eq!(false, tracker.is_space());
    }

    #[test]
    fn peek_and_pop() {
        let mut tracker = StringTracker::new("bogus".to_string(), 0);

        assert_eq!(Some('b'), tracker.peek());
        assert_eq!(5, tracker.chars_left());
        assert_eq!(Some('b'), tracker.pop());
        assert_eq!(4, tracker.chars_left());
        assert_eq!(Some('o'), tracker.read());
        assert_eq!(3, tracker.chars_left());
    }

    #[test]
    fn read_to_end() {
        let mut tracker = StringTracker::new("this is a message from the overlords".to_string(), 0);
    }

    #[test]
    fn empty_tracker() {}
}
