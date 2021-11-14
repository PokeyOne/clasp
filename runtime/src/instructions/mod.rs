use crate::memory::Memory;

use clasp_common::{data_constants::*, data_types::*, instruction_constants::*};

use phf::phf_map;

// Rexport perform function
mod instruction_providers;
pub use instruction_providers::perform;
