use super::types::*;

/// The number of bytes in a word
pub const WORD_SIZE: usize = 8;

#[allow(dead_code)]
pub const REGISTER_COUNT: u64 = 26;
#[allow(dead_code)]
pub const REGISTER_MEMORY_START: MemoryLocation = 0x8000_0000_0000_0000;
#[allow(dead_code)]
pub const RMS: MemoryLocation = REGISTER_MEMORY_START;
#[allow(dead_code)]
pub const GA_LOC: MemoryLocation = RMS + (WORD_SIZE * 0) as u64;
#[allow(dead_code)]
pub const GB_LOC: MemoryLocation = RMS + (WORD_SIZE * 1) as u64;
#[allow(dead_code)]
pub const GC_LOC: MemoryLocation = RMS + (WORD_SIZE * 2) as u64;
#[allow(dead_code)]
pub const GD_LOC: MemoryLocation = RMS + (WORD_SIZE * 3) as u64;
#[allow(dead_code)]
pub const GE_LOC: MemoryLocation = RMS + (WORD_SIZE * 4) as u64;
#[allow(dead_code)]
pub const GF_LOC: MemoryLocation = RMS + (WORD_SIZE * 5) as u64;
#[allow(dead_code)]
pub const GG_LOC: MemoryLocation = RMS + (WORD_SIZE * 6) as u64;
#[allow(dead_code)]
pub const GH_LOC: MemoryLocation = RMS + (WORD_SIZE * 7) as u64;
#[allow(dead_code)]
pub const GI_LOC: MemoryLocation = RMS + (WORD_SIZE * 8) as u64;
#[allow(dead_code)]
pub const GJ_LOC: MemoryLocation = RMS + (WORD_SIZE * 9) as u64;
#[allow(dead_code)]
pub const GK_LOC: MemoryLocation = RMS + (WORD_SIZE * 10) as u64;
#[allow(dead_code)]
pub const GL_LOC: MemoryLocation = RMS + (WORD_SIZE * 11) as u64;
#[allow(dead_code)]
pub const GM_LOC: MemoryLocation = RMS + (WORD_SIZE * 12) as u64;
#[allow(dead_code)]
pub const GN_LOC: MemoryLocation = RMS + (WORD_SIZE * 13) as u64;
#[allow(dead_code)]
pub const GO_LOC: MemoryLocation = RMS + (WORD_SIZE * 14) as u64;
#[allow(dead_code)]
pub const GP_LOC: MemoryLocation = RMS + (WORD_SIZE * 15) as u64;
#[allow(dead_code)]
pub const GQ_LOC: MemoryLocation = RMS + (WORD_SIZE * 16) as u64;
#[allow(dead_code)]
pub const GR_LOC: MemoryLocation = RMS + (WORD_SIZE * 17) as u64;
#[allow(dead_code)]
pub const GS_LOC: MemoryLocation = RMS + (WORD_SIZE * 18) as u64;
#[allow(dead_code)]
pub const GT_LOC: MemoryLocation = RMS + (WORD_SIZE * 19) as u64;
#[allow(dead_code)]
pub const GU_LOC: MemoryLocation = RMS + (WORD_SIZE * 20) as u64;
#[allow(dead_code)]
pub const GV_LOC: MemoryLocation = RMS + (WORD_SIZE * 21) as u64;
#[allow(dead_code)]
pub const GW_LOC: MemoryLocation = RMS + (WORD_SIZE * 22) as u64;
#[allow(dead_code)]
pub const GX_LOC: MemoryLocation = RMS + (WORD_SIZE * 23) as u64;
#[allow(dead_code)]
pub const GY_LOC: MemoryLocation = RMS + (WORD_SIZE * 24) as u64;
#[allow(dead_code)]
pub const GZ_LOC: MemoryLocation = RMS + (WORD_SIZE * 25) as u64;

// TODO: OUT and IN registers. Out will be easiest; Both should be buffered and
//       basically writing to out will just add to buffer until FLO instruction
//       sent. There should also be a register to configure the functionality of
//       the OUT register, such as what's described about, or a non-buffered
//       version, or even a mode that allows direct location, but probably not
//       because location complicated.
