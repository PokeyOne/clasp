use crate::memory::Memory;

use clasp_common::{
    instruction_constants::*,
    data_constants::*,
    data_types::*
};

use phf::phf_map;

// Rexport perform function
mod instruction_providers;
pub use instruction_providers::perform;
