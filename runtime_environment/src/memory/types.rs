use super::constants::*;

// TODO: These definitions have been copied to clasp_common::data_types
//       Remove these and change all references to use those;
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

#[derive(Debug)]
pub enum MemoryErrorType {
    LocationOutOfBounds,
    RegLocationNotAligned,
    LocationNotAligned,
    CannotWriteArrayToRegister,
    #[allow(dead_code)]
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
