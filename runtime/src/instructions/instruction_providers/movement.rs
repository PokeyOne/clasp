use super::*;

pub fn mov_provider(mem: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    *pc += WORD_SIZE as u64;

    let source_location: Word = match program_memory.read(pc.clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;
    let source_value: Word = match mem.read(source_location) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source value word due to memory error type: {:?}",
            t
        )
    };

    let destination_location: Word = match program_memory.read(pc.clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;

    match mem.write(destination_location, source_value) {
        MemoryStatus::Ok => {}
        MemoryStatus::Err(t) => panic!("Moving data to memory threw error: {:?}", t)
    };
}

// TODO: Some of the movr code is copy and paste of the mov code, so could refactor

pub fn movr_provider(mem: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    *pc += WORD_SIZE as u64;

    let source_value: Word = match program_memory.read(pc.clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;

    let destination_location: Word = match program_memory.read(pc.clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;

    match mem.write(destination_location, source_value) {
        MemoryStatus::Ok => {}
        MemoryStatus::Err(t) => panic!("Moving data to memory threw error: {:?}", t)
    };
}
