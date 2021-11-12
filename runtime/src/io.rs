
use crate::memory::Memory;

use clasp_common::io::CCLASP_SIGNATURE;

use std::fs;
use std::io;

pub enum ClaspIOError {
    StandardIOError(io::Error),
    MissingSignature,
    #[allow(dead_code)]
    UnimplementedFeature
}

impl From<io::Error> for ClaspIOError {
    fn from(error: io::Error) -> Self {
        ClaspIOError::StandardIOError(error)
    }
}

pub fn read_cclasp_binary_into_memory(path: &str) -> Result<Memory, ClaspIOError> {
    let data = fs::read(path)?;

    // Verify the clasp file signature
    if CCLASP_SIGNATURE.len() > data.len() {
        return Err(ClaspIOError::MissingSignature);
    }
    for i in 0..(CCLASP_SIGNATURE.len()) {
        if data[i] != CCLASP_SIGNATURE[i] {
            return Err(ClaspIOError::MissingSignature);
        }
    }

    let mut memory = Memory::new((data.len() - CCLASP_SIGNATURE.len()) as u64);
    memory.writes(0u64, &data[(CCLASP_SIGNATURE.len())..data.len()]);

    return Ok(memory);
}
