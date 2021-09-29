use std::convert::TryInto;

/// The number of bytes in a word
const WORD_SIZE: usize = 8;

pub type MemoryLocation = u64;
pub type Word = u64;
pub type Byte = u8;

/// ByteCollection is a trait exclusively meant for the Word type to implement
/// a couple useful functions/methods.
///
/// The main functions of this trait are the from_bytes initializer, and the
/// get_bytes method that converts a u64 word to 8 bytes.
trait ByteCollection {
    /// Convert an array of 8 bytes in to one word.
    fn from_bytes(bytes: &[Byte; WORD_SIZE]) -> Word;
    /// Convert a word into 8 bytes.
    fn get_bytes(&self) -> [Byte; WORD_SIZE];
}

impl ByteCollection for Word {
    fn from_bytes(bytes: &[Byte; WORD_SIZE]) -> Word {
        // TODO: Pretty sure that this check is unnecessary because of the
        //       type definition of the argument being size word_size.
        if bytes.len() != WORD_SIZE {
            panic!()
        }

        let mut result: u64 = 0;

        for byte in bytes {
            result <<= 8;
            result += byte.clone() as Word;
        }

        result
    }

    fn get_bytes(&self) -> [Byte; WORD_SIZE] {
        let mut temp = self.clone();
        let mut result: [Byte; WORD_SIZE] = [0 as Byte; WORD_SIZE];

        for i in 0..8 {
            let value = temp >> (8 * (7 - i));
            let value = value & 0xFF;

            result[i] = value.try_into().unwrap();
        }

        result
    }
}

pub struct Memory {
    memory_size: u64,
    memory: Vec<Byte>
}

impl Memory {
    pub fn read(location: MemoryLocation) -> Word {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_from_bytes() {
        let expected: Word = 0xBACD93FE0102E493;
        let bytes: [Byte; 8] = [0xBA, 0xCD, 0x93, 0xFE, 0x01, 0x02, 0xE4, 0x93];
        let word: Word = Word::from_bytes(&bytes);

        assert_eq!(expected, word);
    }

    #[test]
    fn bytes_from_word() {
        let input_word: Word = 0xBACD93FE0102E493;
        let expected_bytes: [Byte; 8] = [0xBA, 0xCD, 0x93, 0xFE, 0x01, 0x02, 0xE4, 0x93];
        let result: [Byte; 8] = input_word.get_bytes();

        for i in 0..8 {
            assert_eq!(
                expected_bytes[i],
                result[i],
                "Byte {} should match, expected {:#X} -> got {:#X}",
                i,
                expected_bytes[i],
                result[i]
            );
        }
    }
}
