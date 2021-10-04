use crate::memory::types::{Byte, MemoryLocation};
use crate::memory::Memory;

use ascii;
use ascii::AsciiChar;
use std::fs;
use std::io;

pub enum ClaspIOError {
    StandardIOError(io::Error),
    MemoryTooSmall,
    MissingSignature,
    #[allow(dead_code)]
    UnimplementedFeature
}

impl From<io::Error> for ClaspIOError {
    fn from(error: io::Error) -> Self {
        ClaspIOError::StandardIOError(error)
    }
}

/// "CLASP\0" with the zero being the file version. 0 is the only version rn.
/// There is also two reserved bytes after the signature to keep things 8-byte
/// aligned
const CCLASP_SIGNATURE: [Byte; 8] = [
    AsciiChar::C as u8,
    AsciiChar::L as u8,
    AsciiChar::A as u8,
    AsciiChar::S as u8,
    AsciiChar::P as u8,
    0x00u8,
    0x00u8,
    0x00u8
];

pub fn read_cclasp_binary_into_memory(
    memory: &mut Memory,
    address: MemoryLocation,
    path: &str
) -> Result<usize, ClaspIOError> {
    let data = fs::read(path)?;

    // Verify that there is enough memory to load the file into
    if data.len() > memory.len() {
        return Err(ClaspIOError::MemoryTooSmall);
    }

    // Verify the clasp file signature
    if CCLASP_SIGNATURE.len() > data.len() {
        return Err(ClaspIOError::MissingSignature);
    }
    for i in 0..(CCLASP_SIGNATURE.len()) {
        if data[i] != CCLASP_SIGNATURE[i] {
            return Err(ClaspIOError::MissingSignature);
        }
    }

    println!("Loaded data: {:?}", data);

    memory.writes(address, &data[(CCLASP_SIGNATURE.len())..data.len()]);

    println!(
        "Loaded into memory: {:?}",
        &data[(CCLASP_SIGNATURE.len())..data.len()]
    );

    return Ok(data.len() - (CCLASP_SIGNATURE.len()));
}
