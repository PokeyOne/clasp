#[derive(Debug)]
pub enum ConditionalType {
    Equal,
    Greater,
    Lesser
}

impl ConditionalType {
    pub fn perform(&self, a: u64, b: u64) -> bool {
        match self {
            Equal => a == b,
            Greater => a > b,
            Lesser => a < b
        }
    }
}
