use crate::data_types::Byte;

use ascii;
use ascii::AsciiChar;
use std::fs;

/// "CLASP\0" with the zero being the file version. 0 is the only version rn.
/// There is also two reserved bytes after the signature to keep things 8-byte
/// aligned
pub const CCLASP_SIGNATURE: [Byte; 8] = [
    AsciiChar::C as u8, // 0x43
    AsciiChar::L as u8, // 0x4C
    AsciiChar::A as u8, // 0x41
    AsciiChar::S as u8, // 0x53
    AsciiChar::P as u8, // 0x50
    0x00u8,
    0x00u8,
    0x00u8
];

/// Identical to the CCLASP_SGINATURE constant, but in the form of a single
/// word instead of an array
pub const CCLASP_SIGNATURE_WORD: u64 = 0x434C_4153_5000_0000;

pub fn write_binary_file(data: Vec<Byte>, path: &String) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, &data[0..])?;
    Ok(())
}

pub fn print_binary_vec(data: &Vec<Byte>) {
    let mut i = 0;

    for val in data {
        print!("{:02X} ", val);

        if (i + 1) % 8 == 0 {
            print!("\n");
        } else if (i + 1) % 4 == 0 {
            print!(" ");
        }

        i += 1;
    }
}
