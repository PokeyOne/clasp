use std::convert::TryInto;

/// The number of bytes in a word
const WORD_SIZE: usize = 8;

const REGISTER_MEMORY_START: MemoryLocation = 0x8000_0000_0000_0000;
const RMS: MemoryLocation = REGISTER_MEMORY_START;
const GA_LOC: MemoryLocation = RMS + WORD_SIZE * 0;
const GB_LOC: MemoryLocation = RMS + WORD_SIZE * 1;
const GC_LOC: MemoryLocation = RMS + WORD_SIZE * 2;
const GD_LOC: MemoryLocation = RMS + WORD_SIZE * 3;
const GE_LOC: MemoryLocation = RMS + WORD_SIZE * 4;
const GF_LOC: MemoryLocation = RMS + WORD_SIZE * 5;
const GG_LOC: MemoryLocation = RMS + WORD_SIZE * 6;
const GH_LOC: MemoryLocation = RMS + WORD_SIZE * 7;
const GI_LOC: MemoryLocation = RMS + WORD_SIZE * 8;
const GJ_LOC: MemoryLocation = RMS + WORD_SIZE * 9;
const GK_LOC: MemoryLocation = RMS + WORD_SIZE * 10;
const GL_LOC: MemoryLocation = RMS + WORD_SIZE * 11;
const GM_LOC: MemoryLocation = RMS + WORD_SIZE * 12;
const GN_LOC: MemoryLocation = RMS + WORD_SIZE * 13;
const GO_LOC: MemoryLocation = RMS + WORD_SIZE * 14;
const GP_LOC: MemoryLocation = RMS + WORD_SIZE * 15;
const GQ_LOC: MemoryLocation = RMS + WORD_SIZE * 16;
const GR_LOC: MemoryLocation = RMS + WORD_SIZE * 17;
const GS_LOC: MemoryLocation = RMS + WORD_SIZE * 18;
const GT_LOC: MemoryLocation = RMS + WORD_SIZE * 19;
const GU_LOC: MemoryLocation = RMS + WORD_SIZE * 20;
const GV_LOC: MemoryLocation = RMS + WORD_SIZE * 21;
const GW_LOC: MemoryLocation = RMS + WORD_SIZE * 22;
const GX_LOC: MemoryLocation = RMS + WORD_SIZE * 23;
const GY_LOC: MemoryLocation = RMS + WORD_SIZE * 24;
const GZ_LOC: MemoryLocation = RMS + WORD_SIZE * 25;

/// Just an alias for word to be more descriptive when referring to a location
pub type MemoryLocation = u64;
/// The standard unit of the system == 8 bytes == 64 bit
pub type Word = u64;
/// An alias for the u8 type to signal when working with data
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
