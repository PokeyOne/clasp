use super::data_constants::WORD_SIZE;
use std::convert::TryInto;

/// Just an alias for word to be more descriptive when referring to a location
pub type MemoryLocation = u64;
/// The standard unit of the system == 8 bytes == 64 bit
pub type Word = u64;
/// An alias for the u8 type to signal when working with data
pub type Byte = u8;
pub type WordByteArray = [Byte; WORD_SIZE];

pub trait Alignable {
    fn is_aligned(&self) -> bool;
}

impl Alignable for MemoryLocation {
    fn is_aligned(&self) -> bool {
        self % 8 == 0
    }
}

/// ByteCollection is a trait exclusively meant for the Word type to implement
/// a couple useful functions/methods.
///
/// The main functions of this trait are the from_bytes initializer, and the
/// get_bytes method that converts a u64 word to 8 bytes.
pub trait ByteCollection {
    /// Convert an array of 8 bytes in to one word.
    fn from_bytes(bytes: &[Byte; WORD_SIZE]) -> Word;
    /// Convert a vec of 8 bytes in to one word. Anything over 8 bytes is
    /// ignored, and if bytes vec is less than 8, the rest of the bytes are
    /// assumed zero.
    fn from_bytes_v(bytes: &Vec<Byte>) -> Word;
    /// Convert a word into 8 bytes.
    fn to_bytes(&self) -> [Byte; WORD_SIZE];
}

impl ByteCollection for Word {
    fn from_bytes(bytes: &[Byte; WORD_SIZE]) -> Word {
        let mut result: u64 = 0;

        for byte in bytes {
            result <<= 8;
            result += byte.clone() as Word;
        }

        result
    }

    fn from_bytes_v(bytes: &Vec<Byte>) -> Word {
        let mut result: u64 = 0;

        let mut i = 0;
        for byte in bytes {
            if i >= 8 {
                break;
            }

            result <<= 8;
            result += byte.clone() as Word;

            i += 1;
        }

        result
    }

    fn to_bytes(&self) -> [Byte; WORD_SIZE] {
        let temp = self.clone();
        let mut result: [Byte; WORD_SIZE] = [0 as Byte; WORD_SIZE];

        for i in 0..8 {
            let value = temp >> (8 * (7 - i));
            let value = value & 0xFF;

            result[i] = value.try_into().unwrap();
        }

        result
    }
}
