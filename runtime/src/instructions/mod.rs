use crate::memory::constants::*;
use crate::memory::types::Result as MemoryResult;
use crate::memory::types::Status as MemoryStatus;
use crate::memory::types::{MemoryLocation, Word};
use crate::memory::Memory;

pub use clasp_common::instruction_constants::*;

use phf::phf_map;

// Rexport perform function
mod instruction_providers;
pub use instruction_providers::perform;
