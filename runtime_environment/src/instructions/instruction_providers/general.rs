use super::*;

pub fn nop_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
    *pc += WORD_SIZE as u64;
    return;
}

pub fn end_provider(_mem: &mut Memory, pc: &mut MemoryLocation) {
    *pc = 0xFFFF_FFFF_FFFF_FFFFu64;
    return;
}
