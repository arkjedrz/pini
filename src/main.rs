use std::fs;
use std::io::{Error, Read, stdin};
use std::path::PathBuf;

use clap::Parser;
mod parser;
use parser::parse;
mod composer;
use composer::compose;

// TODO:
// - add config file support

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Path to file to process.
    #[arg(short, long, default_value(None))]
    in_path: Option<PathBuf>,
    #[arg(short, long, default_value(None))]
    out_path: Option<PathBuf>,
}

/// Read content from file or stdin.
fn read_content(path: Option<PathBuf>) -> Result<String, Error> {
    if let Some(p) = path {
        let read_result = fs::read_to_string(p);
        match read_result {
            Ok(c) => Ok(c),
            Err(e) => Err(e),
        }
    } else {
        let mut content = String::new();
        let read_result = stdin().read_to_string(&mut content);
        match read_result {
            Ok(_) => Ok(content),
            Err(e) => Err(e),
        }
    }
}

/// Save content to file or stdout.
fn save_content(path: Option<PathBuf>, content: String) -> Result<(), Error> {
    if let Some(p) = path {
        fs::write(p, content)
    } else {
        print!("{}", content);
        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    let input = read_content(args.in_path).expect("Failed to read input file");

    let parsed = parse(input).expect("Failed to parse content");

    let output = compose(parsed).expect("Failed to compose output");

    save_content(args.out_path, output).expect("Failed to save to output file");
}
