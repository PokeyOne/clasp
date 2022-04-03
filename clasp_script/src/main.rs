mod script;

use script::Script;
use std::path::PathBuf;
use clap::Parser;
use clasp_parsing::parsing::parse;
use clasp_parsing::tokenization::tokenize;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Cli {
    path: PathBuf
}

fn main() {
    let args = Cli::parse();

    let source = std::fs::read_to_string(args.path).unwrap();

    let tokens = tokenize(&source).unwrap();
    let ast = parse(tokens).unwrap();

    let script = Script::from_ast(ast);

    script.execute().unwrap();
}
