use clasp_common::instruction_constants::instruction_codes::*;
use clasp_common::data_types::ByteCollection;

use super::{OpProcessError, ArgType, Argument};
use super::utility::process_arg;

pub fn add_process(words: Vec<&str>) -> Result<Vec<u8>, OpProcessError> {
    println!("add: {:?}", &words);

    validate(&words)?;

    Err(OpProcessError::UnimplementedFeature)
}

fn validate(words: &Vec<&str>) -> Result<(), OpProcessError> {
    if (*words).len() != 3 {
        return Err(OpProcessError::WrongNumberOfArguments(
            "Syntax error, expected only 2 arguments for add instruction".to_string()
        ));
    }

    Ok(())
}
