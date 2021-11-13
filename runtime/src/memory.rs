pub mod types;

#[cfg(test)]
mod tests;

use std::convert::TryInto;
use types::{MemResult, MemoryErrorType};
use clasp_common::data_types::{Word, Byte, ByteCollection, MemoryLocation, WordByteArray, Alignable};
use clasp_common::data_constants::{WORD_SIZE, REGISTER_COUNT, RMS};

/// The stored memory for a running VM, including registers and IO. Registers
/// and IO are memory-mapped and stored here as well.
///
/// # Examples
///
/// To initialize a new memory object it would look like this:
/// ```
/// # use clasp_runtime_environment::memory::Memory;
/// # use clasp_runtime_environment::memory;
/// # use clasp_common::data_constants::GA_LOC;
/// let mut mem = Memory::new(12000); // 12kb memory
///
/// mem.read(0); // Read word from 0x0000_0000_0000_0000
/// mem.write(0, 0x1); // Write 0x1 to 0x0000_0000_0000_0000
///
/// let ga = mem.read(GA_LOC); // Read the ga register
/// mem.write(GA_LOC, 0xFF); // Write 0xFF to ga register
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

    pub fn read(&self, location: MemoryLocation) -> MemResult<Word> {
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

    fn read_register(&self, location: MemoryLocation) -> MemResult<Word> {
        if location % WORD_SIZE as u64 != 0 {
            return Err(MemoryErrorType::RegLocationNotAligned);
        }

        let index = (location - RMS) / WORD_SIZE as u64;

        if index > REGISTER_COUNT {
            return Err(MemoryErrorType::LocationOutOfBounds);
        }

        Ok(self.registers[index as usize])
    }

    pub fn writes(&mut self, location: MemoryLocation, value: &[u8]) -> MemResult<()> {
        if location >= RMS {
            return Err(MemoryErrorType::CannotWriteArrayToRegister);
        }

        // If the bytes to write would go out of bounds, then return error
        if location + (value.len() as u64) > (self.memory.len() as u64) {
            return Err(MemoryErrorType::LocationOutOfBounds);
        }

        for i in 0..(value.len()) {
            self.memory[location as usize + i] = value[i];
        }
        Ok(())
    }

    pub fn write(&mut self, location: MemoryLocation, value: Word) -> MemResult<()> {
        if !location.is_aligned() {
            return Err(MemoryErrorType::LocationNotAligned);
        }

        if location >= RMS {
            self.write_register(location, value)
        } else {
            if location > (self.memory.len() as u64) {
                return Err(MemoryErrorType::LocationOutOfBounds);
            }

            let bytes: WordByteArray = value.to_bytes();
            for i in 0..8 {
                self.memory[(location + i) as usize] = bytes[i as usize];
            }

            Ok(())
        }
    }

    fn write_register(&mut self, location: MemoryLocation, value: Word) -> MemResult<()> {
        if !location.is_aligned() {
            return Err(MemoryErrorType::RegLocationNotAligned);
        }

        let index = (location - RMS) / WORD_SIZE as u64;

        if index > REGISTER_COUNT {
            return Err(MemoryErrorType::LocationOutOfBounds);
        }

        self.registers[index as usize] = value;

        Ok(())
    }

    pub fn debug_dump(&self) {
        for i in 0..self.memory.len() {
            if i % 8 == 0 {
                print!("\n{:016X} | ", i);
            } else if i + 1 % 4 == 0 {
                print!(" ");
            }

            print!("{:02X} ", self.memory[i]);
        }

        println!("");
    }
}
