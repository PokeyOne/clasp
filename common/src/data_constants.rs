use super::data_types::*;

/// The number of bytes in a word
pub const WORD_SIZE: usize = 8;

#[allow(dead_code)]
mod register_locations {
    use super::{MemoryLocation, WORD_SIZE};

    pub const REGISTER_COUNT: u64 = 26;
    pub const REGISTER_MEMORY_START: MemoryLocation = 0x8000_0000_0000_0000;
    pub const RMS: MemoryLocation = REGISTER_MEMORY_START;
    pub const GA_LOC: MemoryLocation = RMS + (WORD_SIZE * 0) as u64;
    pub const GB_LOC: MemoryLocation = RMS + (WORD_SIZE * 1) as u64;
    pub const GC_LOC: MemoryLocation = RMS + (WORD_SIZE * 2) as u64;
    pub const GD_LOC: MemoryLocation = RMS + (WORD_SIZE * 3) as u64;
    pub const GE_LOC: MemoryLocation = RMS + (WORD_SIZE * 4) as u64;
    pub const GF_LOC: MemoryLocation = RMS + (WORD_SIZE * 5) as u64;
    pub const GG_LOC: MemoryLocation = RMS + (WORD_SIZE * 6) as u64;
    pub const GH_LOC: MemoryLocation = RMS + (WORD_SIZE * 7) as u64;
    pub const GI_LOC: MemoryLocation = RMS + (WORD_SIZE * 8) as u64;
    pub const GJ_LOC: MemoryLocation = RMS + (WORD_SIZE * 9) as u64;
    pub const GK_LOC: MemoryLocation = RMS + (WORD_SIZE * 10) as u64;
    pub const GL_LOC: MemoryLocation = RMS + (WORD_SIZE * 11) as u64;
    pub const GM_LOC: MemoryLocation = RMS + (WORD_SIZE * 12) as u64;
    pub const GN_LOC: MemoryLocation = RMS + (WORD_SIZE * 13) as u64;
    pub const GO_LOC: MemoryLocation = RMS + (WORD_SIZE * 14) as u64;
    pub const GP_LOC: MemoryLocation = RMS + (WORD_SIZE * 15) as u64;
    pub const GQ_LOC: MemoryLocation = RMS + (WORD_SIZE * 16) as u64;
    pub const GR_LOC: MemoryLocation = RMS + (WORD_SIZE * 17) as u64;
    pub const GS_LOC: MemoryLocation = RMS + (WORD_SIZE * 18) as u64;
    pub const GT_LOC: MemoryLocation = RMS + (WORD_SIZE * 19) as u64;
    pub const GU_LOC: MemoryLocation = RMS + (WORD_SIZE * 20) as u64;
    pub const GV_LOC: MemoryLocation = RMS + (WORD_SIZE * 21) as u64;
    pub const GW_LOC: MemoryLocation = RMS + (WORD_SIZE * 22) as u64;
    pub const GX_LOC: MemoryLocation = RMS + (WORD_SIZE * 23) as u64;
    pub const GY_LOC: MemoryLocation = RMS + (WORD_SIZE * 24) as u64;
    pub const GZ_LOC: MemoryLocation = RMS + (WORD_SIZE * 25) as u64;
}

pub use register_locations::*;

// TODO: actually sort memory using theses constants
pub const PROGRAM_SEGMENT_START: usize = 0b000 << 61;
pub const DATA_SEGMENT_START: MemoryLocation = 0b001 << 61;
pub const STACK_SEGMENT_START: MemoryLocation = 0b010 << 61;
pub const HEAP_SEGMENT_START: MemoryLocation = 0b011 << 61;

// TODO: Consider just an enum Register

pub fn get_register_address(name: &str) -> Option<u64> {
    match name {
        "ga" => Some(GA_LOC),
        "gb" => Some(GB_LOC),
        "gc" => Some(GC_LOC),
        "gd" => Some(GD_LOC),
        "ge" => Some(GE_LOC),
        "gf" => Some(GF_LOC),
        "gg" => Some(GG_LOC),
        "gh" => Some(GH_LOC),
        "gi" => Some(GI_LOC),
        "gj" => Some(GJ_LOC),
        "gk" => Some(GK_LOC),
        "gl" => Some(GL_LOC),
        "gm" => Some(GM_LOC),
        "gn" => Some(GN_LOC),
        "go" => Some(GO_LOC),
        "gp" => Some(GP_LOC),
        "gq" => Some(GQ_LOC),
        "gr" => Some(GR_LOC),
        "gs" => Some(GS_LOC),
        "gt" => Some(GT_LOC),
        "gu" => Some(GU_LOC),
        "gv" => Some(GV_LOC),
        "gw" => Some(GW_LOC),
        "gx" => Some(GX_LOC),
        "gy" => Some(GY_LOC),
        "gz" => Some(GZ_LOC),
        _ => None
    }
}

// TODO: OUT and IN registers. Out will be easiest; Both should be buffered and
//       basically writing to out will just add to buffer until FLO instruction
//       sent. There should also be a register to configure the functionality of
//       the OUT register, such as what's described about, or a non-buffered
//       version, or even a mode that allows direct location, but probably not
//       because location complicated.
