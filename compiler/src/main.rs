use clasp_common::command_line;
use clasp_common::command_line::{CLArg, NamedArgSpec};
use clasm_compiler::compiling;

fn main() {
    let pargs: Vec<CLArg> = command_line::process_args(vec![
        NamedArgSpec::new("--version", false, Some(vec!["-v".to_string()]))
    ]);

    
}
