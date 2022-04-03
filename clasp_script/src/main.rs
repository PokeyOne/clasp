use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Cli {
    path: PathBuf
}

fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
}
