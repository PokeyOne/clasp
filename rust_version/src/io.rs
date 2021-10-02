use crate::memory::Memory;
use crate::memory::types::{MemoryLocation, Byte};

use std::io::prelude::*;
use std::io;
use ascii;
use std::fs;
use ascii::AsciiChar;

pub enum ClaspIOError {
    StandardIOError(io::Error),
    MemoryTooSmall,
    MissingSignature,
    UnimplementedFeature
}

impl From<io::Error> for ClaspIOError {
    fn from(error: io::Error) -> Self {
        ClaspIOError::StandardIOError(error)
    }
}

// "CLASP\0" witht the zero being the file version. 0 is the only version rn
const CCLASP_SIGNATURE: [Byte; 6] = [
    AsciiChar::C as u8,
    AsciiChar::L as u8,
    AsciiChar::A as u8,
    AsciiChar::S as u8,
    AsciiChar::P as u8,
    0x00u8
];

pub fn read_cclasp_binary_into_memory(memory: &Memory, address: MemoryLocation, path: &str) -> Result<usize, ClaspIOError> {
    let data = fs::read(path)?;

    if data.len() > memory.len() {
        return Err(ClaspIOError::MemoryTooSmall);
    }

    for i in 0..(CCLASP_SIGNATURE.len()) {
        if data[i] != CCLASP_SIGNATURE[i] {
            return Err(ClaspIOError::MissingSignature);
        }
    }

    return Err(ClaspIOError::UnimplementedFeature);
}
