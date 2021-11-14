//! This is the main runner of all cclasp files. These are the files that have
//! alright been compiled all the way down to binary.

mod instructions;
mod io;
mod memory;
mod running;

use io::ClaspIOError::*;


use clasp_common::command_line::{process_args, CLArg, NamedArgSpec};


/// The default value if not specified on command line of if the program should
/// dump all memory out to the standard output upon program completion. Useful
/// for debugging instruction development. Safe to change. Should be false in
/// release.
const DEFAULT_SHOW_DUMP: bool = true;
// TODO: const DEFAULT_MEMORY_SIZE, plus an argument to set the size.

fn main() {
    let args: Vec<CLArg> = process_args(vec![
        NamedArgSpec::new("--nodump", false, None),
        NamedArgSpec::new("--dump", false, None),
    ]);

    // This will be set if the command line argument is supplied, but left as
    // an optional so that an error can be thrown later if not given. This is
    // the path to the file to read the program from.
    let mut maybe_path: Option<String> = None;

    // True when the whole memory should be written to standard output upon
    // program completion
    let mut should_show_dump: bool = DEFAULT_SHOW_DUMP;

    // Loop through the arguments and set thing appropriately
    for arg in args {
        if arg.is_anonymous() {
            maybe_path = Some(arg.value);
        } else {
            match arg.name {
                Some(val) => match val.as_str() {
                    "--nodump" => should_show_dump = false,
                    "--dump" => should_show_dump = true,
                    _ => panic!("Unknown argument {}", val)
                },
                None => {} // Do nothing because this shouldn't happen
            };
        }
    }

    // Panic if we were never given a path, and de-optionalize it.
    let path: String = match maybe_path {
        Some(val) => val,
        None => panic!("No path provided to read program from.")
    };

    let program_memory = match io::read_cclasp_binary_into_memory(&path) {
        Ok(val) => val,
        Err(cioe) => match cioe {
            MissingSignature => {
                panic!("The file you tried to run is either not a cclasp file or is corrupted")
            }
            UnimplementedFeature => panic!(
                "The cclasp file you tried to run contains features that are not yet implemented"
            ),
            StandardIOError(err) => panic!("IO Error occurred while opening file: {:?}", err)
        }
    };

    // TODO: Variable container size
    let memory = match running::run_program(program_memory, 12000) {
        Ok(mem) => mem,
        Err(_msg) => panic!("Currupted program")
    };

    if should_show_dump {
        memory.debug_dump();
    }

    println!("Hello, world!");
}
