use std::convert::TryInto;

pub type MemoryLocation = u64;
pub type Word = u64;
pub type Byte = u8;

trait ByteCollection {
    fn from_bytes(bytes: Vec<Byte>) -> Word;
    fn get_bytes(&self) -> Vec<Byte>;
}

impl ByteCollection for Word {
    fn from_bytes(bytes: Vec<Byte>) -> Word {
        if bytes.len() != 4 {
            panic!() // TODO: Handle this properly
        }

        ((((((bytes[0] << 8) + bytes[1]) << 8) + bytes[2]) << 8) + bytes[3]).into()
    }

    fn get_bytes(&self) -> Vec<Byte> {
        let mut temp = self.clone();
        let mut result: Vec<Byte> = Vec::new();

        // This is going to require casting
        result.push((temp % 256).try_into().unwrap());
        temp = temp >> 8;
        result.push((temp % 256).try_into().unwrap());
        temp = temp >> 8;
        result.push((temp % 256).try_into().unwrap());
        temp = temp >> 8;
        result.push(temp.try_into().unwrap());

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
