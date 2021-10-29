mod compiling;

use clasm_compiler::compiling as clasm_compiling;
use clasp_common::command_line;
use clasp_common::command_line::{CLArg, NamedArgSpec};
use clasp_common::version_constants::VERSION_STRING;
use std::fs;

fn read_cl_args() -> Result<(String, String), Option<String>> {
    let args: Vec<CLArg> = command_line::process_args(vec![NamedArgSpec::new(
        "--version",
        false,
        Some(vec!["-v".to_string()])
    )]);

    let mut output_path = "./a.out".to_string();
    let mut input_path: Option<String> = None;

    for arg in args {
        match arg.name {
            Some(val) => match (&val) as &str {
                "--version" => {
                    println!("Clasp Compiler Version {}\n", VERSION_STRING);
                    return Err(None);
                }
                _ => {}
            },
            None => input_path = Some(arg.value)
        };
    }

    match input_path {
        Some(val) => Ok((val, output_path)),
        None => Err(Some("Input path not supplied".to_string()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (input_path, output_path) = match read_cl_args() {
        Ok(val) => val,
        Err(msg) => match msg {
            Some(msge) => panic!("{}", msge),
            None => return Ok(())
        }
    };

    let file_content = fs::read_to_string(input_path)?;
    let resulting_assembly: String = compiling::compile_text(file_content);
    // TODO: Stop here if assembly-only option is given
    let resulting_binary: Vec<u8> = clasm_compiling::compile_text(resulting_assembly);

    fs::write(output_path, &resulting_binary)?;

    Ok(())
}
