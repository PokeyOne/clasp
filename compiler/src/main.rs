mod compiling;
mod run_options;

use clasm_compiler::compiling as clasm_compiling;
use clasp_common::command_line;
use clasp_common::command_line::{CLArg, NamedArgSpec};
use clasp_common::version_constants::VERSION_STRING;
use run_options::{factory::RunOptionsFactory, OutputFormat, RunOptions};
use std::fs;

fn help_text() -> String {
    let mut help_text = String::new();
    help_text.push_str("Usage: clasmc [options] <file>\n");
    help_text.push_str("Options:\n");
    help_text.push_str("  -h, --help                 Print this help text\n");
    help_text.push_str("  -v, --version              Print the version number\n");
    // TODO: Commented ones are not implemented but should be.
    //help_text.push_str("  -o, --output <file>        Write the output to <file>\n");
    //help_text.push_str("  -d, --debug                Enable debug symbols\n");
    //help_text.push_str("  -w, --warnings             Enable warnings\n");
    //help_text.push_str("  -W, --no-warnings          Disable warnings\n");
    help_text
        .push_str("  -S, --assembly             Only compile source to assembly, not executable\n");

    help_text
}

fn read_cl_args() -> Result<RunOptions, Option<String>> {
    let args: Vec<CLArg> = command_line::process_args(vec![
        NamedArgSpec::new("--version", false, Some(vec!["-v".to_string()])),
        NamedArgSpec::new(
            "--help",
            false,
            Some(vec!["-h".to_string(), "-?".to_string()])
        ),
        NamedArgSpec::new("--assembly", false, Some(vec!["-S".to_string()])),
    ]);

    let mut run_options_factory = RunOptionsFactory::new();
    let mut should_continue = true;

    for arg in args {
        match arg.name {
            Some(val) => match (&val) as &str {
                "--version" => {
                    println!("Clasp Compiler Version {}\n", VERSION_STRING);
                    should_continue = false;
                }
                "--assembly" => {
                    run_options_factory.set_output_format(OutputFormat::Assembly);
                }
                "--help" => {
                    println!("{}", help_text());
                    should_continue = false;
                }
                _ => {}
            },
            None => run_options_factory.set_input_path(arg.value)
        };
    }

    if !should_continue {
        return Err(None);
    }

    match run_options_factory.construct() {
        Ok(val) => Ok(val),
        Err(val) => Err(Some(val))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let run_options = match read_cl_args() {
        Ok(val) => val,
        Err(msg) => match msg {
            Some(msge) => panic!("{}", msge),
            None => return Ok(())
        }
    };

    let file_content = fs::read_to_string(run_options.input_path())?;
    let resulting_assembly: String = compiling::compile_text(file_content);
    // TODO: Stop here if assembly-only option is given
    let resulting_binary: Vec<u8> = clasm_compiling::compile_text(resulting_assembly);

    fs::write(run_options.output_path(), &resulting_binary)?;

    Ok(())
}
