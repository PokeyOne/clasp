use crate::memory::types::{MemoryLocation};
use crate::memory::Memory;

use clasp_common::io::CCLASP_SIGNATURE;

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
