pub mod command_line;
pub mod data_constants;
pub mod data_types;
pub mod instruction_constants;
pub mod io;
pub mod version_constants;
pub mod conditional_type;
pub mod maths;
pub mod parsing;

// TODO: remove this base tests module. Use the tests folder next to src for
//       integration tests, and use submodules of the actual modules for unit tests
#[cfg(test)]
mod tests;
