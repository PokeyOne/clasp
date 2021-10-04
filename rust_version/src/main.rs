mod instructions;
mod io;
mod memory;

use std::env;

use io::ClaspIOError;
use io::ClaspIOError::*;
use memory::types::{MemoryErrorType, MemoryLocation};
use memory::Memory;

fn main() {
    // TODO: command line arguments for things like memory size and the like
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected command with only one argument with the path of a cclasp file");
    }

    let mut memory: Memory = Memory::new(12000);
    let mut program_counter: MemoryLocation = 0;
    let path: &str = &args[1];

    match io::read_cclasp_binary_into_memory(&mut memory, program_counter, &path) {
        Ok(bytes_read) => println!("Loaded file '{}' into {} bytes of memory", path, bytes_read),
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
    }

    loop {
        let inst = match memory.read(program_counter) {
            memory::types::Result::Ok(val) => val,
            memory::types::Result::Err(t) => panic!("Instruction read error: {:?}", t)
        };

        println!("Evaluating instruction {:X}", inst);
        instructions::perform(inst, &mut memory, &mut program_counter);

        if program_counter == 0xFFFF_FFFF_FFFF_FFFFu64 {
            break;
        }
    }

    memory.debug_dump();
    println!("Hello, world!");
}
