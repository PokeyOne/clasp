use super::constants::*;

/// Just an alias for word to be more descriptive when referring to a location
pub type MemoryLocation = u64;
/// The standard unit of the system == 8 bytes == 64 bit
pub type Word = u64;
/// An alias for the u8 type to signal when working with data
pub type Byte = u8;
pub type WordByteArray = [Byte; WORD_SIZE];

pub enum Result<T> {
    Ok(T),
    Err(MemoryErrorType)
}

impl<T> Result<T> {
    pub fn expect(&self, msg: &str) -> &T {
        match self {
            Result::Ok(t) => t,
            Result::Err(t) => {
                println!("{}", msg);
                panic!();
            }
        }
    }
}

#[derive(Debug)]
pub enum MemoryErrorType {
    LocationOutOfBounds,
    RegLocationNotAligned,
    LocationNotAligned,
    CannotWriteArrayToRegister,
    FunctionalityNotImplemented
}

pub enum Status {
    Ok,
    Err(MemoryErrorType)
}

pub trait Alignable {
    fn is_aligned(&self) -> bool;
}

impl Alignable for MemoryLocation {
    fn is_aligned(&self) -> bool {
        self % 8 == 0
    }
}