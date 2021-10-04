pub mod constants;
pub mod types;

use constants::*;
use std::convert::TryInto;
use types::Result::*;
use types::*;

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

/// The stored memory for a running VM, including registers and IO. Registers
/// and IO are memory-mapped and stored here as well.
///
/// # Examples
///
/// To initialize a new memory object it would look like this:
/// ```
/// let mem = Memory::new(12000); // 12kb memory
///
/// mem.read(0); // Read word from 0x0000_0000_0000_0000
/// mem.write(0, 0x1); // Write 0x1 to 0x0000_0000_0000_0000
///
/// let ga = mem.read(Memory::constants::GA_LOC); // Read the ga register
/// mem.write(Memory::constants::GA_LOC, 0xFF); // Write 0xFF to ga register
/// ```
/// _note_ that the above example does not account for the Result object that
/// is returned from the read methods, and the Status enum that is returned
/// from the write method. This can be handled in standard ways.
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

    pub fn len(&self) -> usize {
        self.size()
    }

    pub fn size(&self) -> usize {
        self.memory_size.try_into().unwrap()
    }

    pub fn read(&self, location: MemoryLocation) -> Result<Word> {
        if location >= RMS {
            self.read_register(location)
        } else {
            if location + 8 > (self.memory.len() as u64) {
                return Err(MemoryErrorType::LocationOutOfBounds);
            }

            // Might be more efficient to store memory as words and
            // editing bytes takes longer, because words will be most common
            let mut bytes: WordByteArray = [0; WORD_SIZE];
            for i in 0..8 {
                bytes[i] = self.memory[i + location as usize];
            }

            let ans = Word::from_bytes(&bytes);
            Ok(ans)
        }
    }

    fn read_register(&self, location: MemoryLocation) -> Result<Word> {
        if location % WORD_SIZE as u64 != 0 {
            return Err(MemoryErrorType::RegLocationNotAligned);
        }

        let index = (location - RMS) / WORD_SIZE as u64;

        if index > REGISTER_COUNT {
            return Err(MemoryErrorType::LocationOutOfBounds);
        }

        Ok(self.registers[index as usize])
    }

    pub fn writes(&mut self, location: MemoryLocation, value: &[u8]) -> Status {
        if location >= RMS {
            return Status::Err(MemoryErrorType::CannotWriteArrayToRegister);
        }

        // If the bytes to write would go out of bounds, then return error
        if location + (value.len() as u64) > (self.memory.len() as u64) {
            return Status::Err(MemoryErrorType::LocationOutOfBounds);
        }

        for i in 0..(value.len()) {
            self.memory[location as usize + i] = value[i];
        }
        Status::Ok
    }

    pub fn write(&mut self, location: MemoryLocation, value: Word) -> Status {
        if !location.is_aligned() {
            return Status::Err(MemoryErrorType::LocationNotAligned);
        }

        if location >= RMS {
            self.write_register(location, value)
        } else {
            if location > (self.memory.len() as u64) {
                return Status::Err(MemoryErrorType::LocationOutOfBounds);
            }

            let bytes: WordByteArray = value.get_bytes();
            for i in 0..8 {
                self.memory[(location + i) as usize] = bytes[i as usize];
            }

            Status::Ok
        }
    }

    fn write_register(&mut self, location: MemoryLocation, value: Word) -> Status {
        if !location.is_aligned() {
            return Status::Err(MemoryErrorType::RegLocationNotAligned);
        }

        let index = (location - RMS) / WORD_SIZE as u64;

        if index > REGISTER_COUNT {
            return Status::Err(MemoryErrorType::LocationOutOfBounds);
        }

        self.registers[index as usize] = value;

        Status::Ok
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
                expected_bytes[i], result[i],
                "Byte {} should match, expected {:#X} -> got {:#X}",
                i, expected_bytes[i], result[i]
            );
        }
    }

    #[test]
    fn initial_memory_should_be_zero_and_readable() {
        let mut memory: Memory = Memory::new(1000); // 1kb

        for i in 0..(1000 - 8) {
            println!("{}", i);
            let value: Word = match memory.read(i as MemoryLocation) {
                Ok(value) => value,
                Err(err_type) => {
                    panic!();
                }
            };

            assert_eq!(0, value);
        }

        assert_eq!(memory.size(), 1000);
    }

    #[test]
    fn writing_to_memory() {
        let mut memory: Memory = Memory::new(8);

        memory.write(0, 0x3F);

        assert_eq!(
            0x3F,
            match memory.read(0) {
                Ok(value) => value,
                Err(err_type) => {
                    panic!();
                }
            }
        );
    }

    #[test]
    fn writing_and_reading_to_register() {
        let mut memory: Memory = Memory::new(16);

        let test_value = 0x0123456789ABCDEF;
        match memory.write(GA_LOC, test_value) {
            Status::Ok => {}
            Status::Err(err_type) => {
                panic!("Could not write to memory with error: {:?}", err_type);
            }
        };

        assert_eq!(
            test_value,
            match memory.read(GA_LOC) {
                Ok(value) => value,
                Err(err_type) => {
                    panic!();
                }
            }
        );
    }
}
