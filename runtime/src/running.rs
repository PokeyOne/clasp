use crate::instructions;
use crate::memory;
use crate::memory::Memory;
use clasp_common::data_types::MemoryLocation;

/// Runs the given compiled program. The binary program must be loaded and
/// stripped of its file signature, then loaded into a Memory object before
/// being passed into this file. The container size is the number of bytes of
/// memory space that this program should have to run.
pub fn run_program(mut program_memory: Memory, container_size: u64) -> Result<Memory, String> {
    let mut memory: Memory = Memory::new(container_size);
    let mut program_counter: MemoryLocation = 0;

    loop {
        let inst = match program_memory.read(program_counter) {
            Ok(val) => val,
            Err(t) => {
                return Err(format!("Instruction read error: {:?}", t).to_string())
            }
        };

        instructions::perform(inst, &mut memory, &mut program_memory, &mut program_counter);

        if program_counter == 0xFFFF_FFFF_FFFF_FFFFu64 {
            return Ok(memory);
        }
    }
}
