use crate::data_types::Byte;

use ascii;
use ascii::AsciiChar;
use std::fs;

/// "CLASP\0" with the zero being the file version. 0 is the only version rn.
/// There is also two reserved bytes after the signature to keep things 8-byte
/// aligned
pub const CCLASP_SIGNATURE: [Byte; 8] = [
    AsciiChar::C as u8,
    AsciiChar::L as u8,
    AsciiChar::A as u8,
    AsciiChar::S as u8,
    AsciiChar::P as u8,
    0x00u8,
    0x00u8,
    0x00u8
];

pub fn write_binary_file(data: Vec<Byte>, path: &String) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, &data[0..])?;
    Ok(())
}
