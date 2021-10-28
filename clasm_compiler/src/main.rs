use clasp_common::command_line;
use clasp_common::command_line::{CLArg, NamedArgSpec};
use clasp_common::version_constants::VERSION_STRING;
use std::fs;

mod text_processing;
mod label;

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

    let file_content = fs::read_to_string(&input_path)?;
    let mut resulting_byte_code: Vec<u8> = Vec::new();

    // Append the clasp file signature to the data buffer
    for sig_byte in clasp_common::io::CCLASP_SIGNATURE {
        resulting_byte_code.push(sig_byte);
    }

    let mut labels = LabelCollection::new();

    let mut line_index = 0;
    for line in file_content.lines() {
        line_index += 1;

        // TODO: Possible run this label check on trimmed line.
        if line.chars().nth(0) == Some(':') {
            labels.insert(line.to_string(), resulting_byte_code.len() as u64);
            continue;
        }

        let mut important_words: Vec<&str> = Vec::new();

        for word in line.split(' ') {
            if word == ";;" {
                break;
            }

            let trimmed = word.trim();

            if trimmed == "" {
                continue;
            }

            important_words.push(trimmed);
        }

        if important_words.len() == 0 {
            continue;
        }

        let byte_code_result = match important_words[0] {
            "nop" => text_processing::nop_process(important_words),
            "mov" => text_processing::mov_process(important_words),
            "outr" => text_processing::outr_process(important_words),
            "end" => text_processing::end_process(important_words),
            "add" => text_processing::add_process(important_words),
            "sub" => text_processing::sub_process(important_words),
            "mul" => text_processing::mul_process(important_words),
            "div" => text_processing::div_process(important_words),
            "pow" => text_processing::pow_process(important_words),
            _ => panic!(
                "Syntax error, unexpected instruction at line {}",
                line_index
            )
        };

        let mut byte_code = match byte_code_result {
            Ok(val) => val,
            Err(err) => panic!("line {}: {:?}", line_index, err)
        };

        resulting_byte_code.append(&mut byte_code);
    }

    println!(
        "Compiled to {} ({:X}) raw bytes: {:?}",
        resulting_byte_code.len(),
        resulting_byte_code.len(),
        resulting_byte_code
    );

    println!("Collected {} labels", labels.size());
    labels.print_ordered_list();

    if dump_byte_code {
        clasp_common::io::print_binary_vec(&resulting_byte_code);
    }

    match clasp_common::io::write_binary_file(resulting_byte_code, &output_file_location) {
        Ok(_) => println!("Successfully wrote to file {}", &output_file_location),
        Err(e) => panic!("{:?}", e)
    };

    return Ok(());
}
