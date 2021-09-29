use std::convert::TryInto;

/// The number of bytes in a word
const WORD_SIZE: usize = 8;

#[allow(dead_code)]
const REGISTER_COUNT: u64 = 26;
#[allow(dead_code)]
const REGISTER_MEMORY_START: MemoryLocation = 0x8000_0000_0000_0000;
#[allow(dead_code)]
const RMS: MemoryLocation = REGISTER_MEMORY_START;
#[allow(dead_code)]
const GA_LOC: MemoryLocation = RMS + (WORD_SIZE * 0) as u64;
#[allow(dead_code)]
const GB_LOC: MemoryLocation = RMS + (WORD_SIZE * 1) as u64;
#[allow(dead_code)]
const GC_LOC: MemoryLocation = RMS + (WORD_SIZE * 2) as u64;
#[allow(dead_code)]
const GD_LOC: MemoryLocation = RMS + (WORD_SIZE * 3) as u64;
#[allow(dead_code)]
const GE_LOC: MemoryLocation = RMS + (WORD_SIZE * 4) as u64;
#[allow(dead_code)]
const GF_LOC: MemoryLocation = RMS + (WORD_SIZE * 5) as u64;
#[allow(dead_code)]
const GG_LOC: MemoryLocation = RMS + (WORD_SIZE * 6) as u64;
#[allow(dead_code)]
const GH_LOC: MemoryLocation = RMS + (WORD_SIZE * 7) as u64;
#[allow(dead_code)]
const GI_LOC: MemoryLocation = RMS + (WORD_SIZE * 8) as u64;
#[allow(dead_code)]
const GJ_LOC: MemoryLocation = RMS + (WORD_SIZE * 9) as u64;
#[allow(dead_code)]
const GK_LOC: MemoryLocation = RMS + (WORD_SIZE * 10) as u64;
#[allow(dead_code)]
const GL_LOC: MemoryLocation = RMS + (WORD_SIZE * 11) as u64;
#[allow(dead_code)]
const GM_LOC: MemoryLocation = RMS + (WORD_SIZE * 12) as u64;
#[allow(dead_code)]
const GN_LOC: MemoryLocation = RMS + (WORD_SIZE * 13) as u64;
#[allow(dead_code)]
const GO_LOC: MemoryLocation = RMS + (WORD_SIZE * 14) as u64;
#[allow(dead_code)]
const GP_LOC: MemoryLocation = RMS + (WORD_SIZE * 15) as u64;
#[allow(dead_code)]
const GQ_LOC: MemoryLocation = RMS + (WORD_SIZE * 16) as u64;
#[allow(dead_code)]
const GR_LOC: MemoryLocation = RMS + (WORD_SIZE * 17) as u64;
#[allow(dead_code)]
const GS_LOC: MemoryLocation = RMS + (WORD_SIZE * 18) as u64;
#[allow(dead_code)]
const GT_LOC: MemoryLocation = RMS + (WORD_SIZE * 19) as u64;
#[allow(dead_code)]
const GU_LOC: MemoryLocation = RMS + (WORD_SIZE * 20) as u64;
#[allow(dead_code)]
const GV_LOC: MemoryLocation = RMS + (WORD_SIZE * 21) as u64;
#[allow(dead_code)]
const GW_LOC: MemoryLocation = RMS + (WORD_SIZE * 22) as u64;
#[allow(dead_code)]
const GX_LOC: MemoryLocation = RMS + (WORD_SIZE * 23) as u64;
#[allow(dead_code)]
const GY_LOC: MemoryLocation = RMS + (WORD_SIZE * 24) as u64;
#[allow(dead_code)]
const GZ_LOC: MemoryLocation = RMS + (WORD_SIZE * 25) as u64;

/// Just an alias for word to be more descriptive when referring to a location
pub type MemoryLocation = u64;
/// The standard unit of the system == 8 bytes == 64 bit
pub type Word = u64;
/// An alias for the u8 type to signal when working with data
pub type Byte = u8;
pub type WordByteArray = [Byte; WORD_SIZE];

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

pub struct Memory {
    memory_size: u64,
    memory: Vec<Byte>,
    registers: [Word; REGISTER_COUNT as usize]
}

impl Memory {
    pub fn new(size: u64) -> Memory {
        let mut memory: Vec<Byte> = Vec::new();
        memory.resize(size as usize, 0 as Byte);

        Memory {
            memory_size: size,
            memory: memory,
            registers: [0 as Word; REGISTER_COUNT as usize]
        }
    }

    pub fn read(&self, location: MemoryLocation) -> Word {
        if location >= RMS {
            self.read_register(location)
        } else {
            // Might be more efficient to store memory as words and
            // editing bytes takes longer, because words will be most common
            let mut bytes: WordByteArray = [0; WORD_SIZE];
            for i in 0..8 {
                bytes[i] = self.memory[i + location as usize];
            }

            Word::from_bytes(&bytes)
        }
    }

    fn read_register(&self, location: MemoryLocation) -> Word {
        if location % WORD_SIZE as u64 != 0 {
            panic!() // TODO: Actually handle this. Some sort of RuntimeError type. Maybe a custom Result for runtime environment
        }

        let index = (location - RMS) / WORD_SIZE as u64;

        // TODO: Also return error for out of bounds
        self.registers[index as usize]
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
