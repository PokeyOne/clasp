pub mod command_line;
pub mod data_constants;
pub mod data_types;
pub mod instruction_constants;
pub mod io;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}