use super::*;

// TODO: Move raw instruction for moving a litteral
pub fn mov_provider(mem: &mut Memory, pc: &mut MemoryLocation) {
    *pc += WORD_SIZE as u64;

    let source_location: Word = match (*mem).read((*pc).clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;
    let source_value: Word = match (*mem).read(source_location) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source value word due to memory error type: {:?}",
            t
        )
    };

    let destination_location: Word = match (*mem).read((*pc).clone()) {
        MemoryResult::Ok(val) => val,
        MemoryResult::Err(t) => panic!(
            "Could not read move instruction source location word due to memory error type: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;

    println!(
        "About to move {:016X} from {:016X} to {:016X}",
        source_value, source_location, destination_location
    );
    match (*mem).write(destination_location, source_value) {
        MemoryStatus::Ok => {}
        MemoryStatus::Err(t) => panic!("Moving data to memory threw error: {:?}", t)
    };
    return;
}
