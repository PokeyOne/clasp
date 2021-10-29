mod compiling;

use clasp_common::command_line;
use clasp_common::command_line::{CLArg, NamedArgSpec};
use clasm_compiler::compiling as clasm_compiling;

fn read_cl_args() -> Result<(String, String), Option<String>> {
    let args: Vec<CLArg> = command_line::process_args(vec![
        NamedArgSpec::new("--version", false, Some(vec!["-v".to_string()]))
    ]);

    let mut output_path = "./a.out".to_string();
    let mut input_path: Option<String> = None;

    for arg in args {
        match arg.name {
            Some(val) => match (&val) as &str {
                "--version" => {
                    println!("Version ??");
                    return Err(None);
                }
                _ => {}
            }
            None => input_path = Some(arg.value)
        };
    }

    match input_path {
        Some(val) => Ok((val, output_path)),
        None => Err(Some("Input path not supplied".to_string()))
    }
}

fn main() {
    let (input_path, output_path) = match read_cl_args() {
        Ok(val) => val,
        Err(msg) => match msg {
            Some(msge) => panic!("{}", msge),
            None => return
        }
    };

    println!("Got input path of {} and output of {}", input_path, output_path);
}
