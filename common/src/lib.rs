pub mod command_line;
pub mod conditional_type;
pub mod data_constants;
pub mod data_types;
pub mod instruction_constants;
pub mod io;
pub mod test_helpers;
pub mod version_constants;

// TODO: remove this base tests module. Use the tests folder next to src for
//       integration tests, and use submodules of the actual modules for unit tests
#[cfg(test)]
mod tests;
