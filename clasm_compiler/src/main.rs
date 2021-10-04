use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Command line arguments for things like output file location
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected command with only one argument with the path of the clasm source file");
    }

    let file_content = fs::read_to_string(&args[1])?;

    for line in file_content.lines() {
        println!("line: {}", line);
    }

    return Ok(());
}
