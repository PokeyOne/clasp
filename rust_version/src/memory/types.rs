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

pub enum MemoryErrorType {
    LocationOutOfBounds,
    RegLocationNotAligned
}
