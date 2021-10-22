//! This is the main runner of all cclasp files. These are the files that have
//! alright been compiled all the way down to binary.

mod instructions;
mod io;
mod memory;

use std::env;

use io::ClaspIOError::*;
use memory::types::MemoryLocation;
use memory::Memory;

use clasp_common::command_line::{CLArg, NamedArgSpec, process_args};

/// The default value if not specified on command line of if the program should
/// dump all memory out to the standard output upon program completion. Useful
/// for debugging instruction development. Safe to change. Should be false in
/// release.
const DEFAULT_SHOW_DUMP: bool = true;
// TODO: const DEFAULT_MEMORY_SIZE, plus an argument to set the size.

fn main() {
    let args: Vec<CLArg> = process_args(vec![
        NamedArgSpec::new("--nodump", false, None),
        NamedArgSpec::new("--dump", false, None)
    ]);

    /// This will be set if the command line argument is supplied, but left as
    /// an optional so that an error can be thrown later if not given. This is
    /// the path to the file to read the program from.
    let mut maybe_path: Option<String> = None;

    /// True when the whole memory should be written to standard output upon
    /// program completion
    let mut should_show_dump: bool = DEFAULT_SHOW_DUMP;

    // Loop through the arguments and set thing appropriately
    for arg in args {
        if arg.is_anonymous() {
            maybe_path = Some(arg.value);
        } else {
            match arg.name {
                Some(val) => {
                    match val.as_str() {
                        "--nodump" => should_show_dump = false,
                        "--dump" => should_show_dump = true,
                        _ => panic!("Unknown argument {}", val)
                    }
                },
                None => {} // Do nothing because this shouldn't happen
            };
        }
    }

    let mut memory: Memory = Memory::new(12000);
    let mut program_counter: MemoryLocation = 0;
    // Panic if we were never given a path, and de-optionalize it.
    let path: String = match maybe_path {
        Some(val) => val,
        None => panic!("No path provided to read program from.")
    };

    let mut program_memory = match io::read_cclasp_binary_into_memory(&path) {
        Ok(val) => val,
        Err(cioe) => match cioe {
            MemoryTooSmall => {
                panic!("Memory too small for program; increase memory size to solve.")
            }
            MissingSignature => {
                panic!("The file you tried to run is either not a cclasp file or is corrupted")
            }
            UnimplementedFeature => panic!(
                "The cclasp file you tried to run contains features that are not yet implemented"
            ),
            StandardIOError(err) => panic!("IO Error occurred while opening file: {:?}", err)
        }
    };

    loop {
        let inst = match program_memory.read(program_counter) {
            memory::types::Result::Ok(val) => val,
            memory::types::Result::Err(t) => panic!("Instruction read error: {:?}", t)
        };

        instructions::perform(inst, &mut memory, &mut program_memory, &mut program_counter);

        if program_counter == 0xFFFF_FFFF_FFFF_FFFFu64 {
            break;
        }
    }

    if should_show_dump {
        memory.debug_dump();
    }

    println!("Hello, world!");
}
