use clasp_common::command_line;
use clasp_common::command_line::{CLArg, NamedArgSpec};
use clasp_common::data_constants::WORD_SIZE;
use clasp_common::version_constants::VERSION_STRING;
use std::fs;
use std::time::Instant;

mod compiling;
mod label;
mod text_processing;

#[cfg(test)]
mod tests;

use label::LabelCollection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pargs: Vec<CLArg> = command_line::process_args(vec![
        NamedArgSpec::new("--output", true, Some(vec!["-o".to_string()])),
        NamedArgSpec::new("--hello", false, None),
        NamedArgSpec::new("--version", false, Some(vec!["-v".to_string()])),
        NamedArgSpec::new("--nodump", false, None),
        NamedArgSpec::new("--dump", false, None),
    ]);
    let mut output_file_location: String = "./a.out".to_string();
    let mut input_path: String = String::new();
    let mut dump_byte_code: bool = false;

    for parg in pargs {
        println!("arugment: {:?}", parg);
        if parg.is_anonymous() {
            input_path = parg.value;
        } else {
            match parg.name {
                None => {}
                Some(n) => {
                    if n == "--output" {
                        output_file_location = parg.value;
                    } else if n == "--hello" {
                        println!("Hello, World");
                    } else if n == "--version" {
                        println!("Clasm Compiler Version {}\n", VERSION_STRING);

                        return Ok(());
                    } else if n == "--nodump" {
                        dump_byte_code = false;
                    } else if n == "--dump" {
                        dump_byte_code = true;
                    }
                }
            }
        }
    }

    if input_path == "" {
        panic!("Must suply input path");
    }

    let start_time = Instant::now();

    let file_content = fs::read_to_string(&input_path)?;
    let mut resulting_byte_code: Vec<u8> = compiling::compile_text(file_content);

    let elapsed_time = start_time.elapsed();
    println!(
        "Compiled to {} ({:#X}) bytes in {}.{:06}s or {}ms",
        resulting_byte_code.len(),
        resulting_byte_code.len(),
        elapsed_time.as_secs(),
        elapsed_time.subsec_micros(),
        elapsed_time.as_millis()
    );

    if dump_byte_code {
        clasp_common::io::print_binary_vec(&resulting_byte_code);
    }

    match clasp_common::io::write_binary_file(resulting_byte_code, &output_file_location) {
        Ok(_) => println!("Successfully wrote to file {}", &output_file_location),
        Err(e) => panic!("{:?}", e)
    };

    return Ok(());
}
