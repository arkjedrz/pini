mod composer;
mod format_config;
mod parser;

use std::env::current_dir;
use std::fs;
use std::io::{Error, Read, stdin};
use std::path::PathBuf;

use clap::Parser;
use composer::compose;
use format_config::FormatConfig;
use parser::parse;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Input path.
    #[arg(short, long, default_value(None))]
    in_path: Option<PathBuf>,
    /// Output path.
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

/// Find config file.
fn find_config_file() -> Option<PathBuf> {
    let cwd = current_dir().unwrap();
    let mut curr_dir = Some(cwd.as_path());
    while let Some(p) = curr_dir {
        let config_path = p.join(".pini");
        if config_path.exists() {
            return Some(config_path);
        }

        // Change processed directory.
        curr_dir = p.parent();
    }

    None
}

fn main() {
    // Parse arguments.
    let args = Args::parse();

    // Find and load config.
    let config_path = find_config_file();
    let config = match config_path {
        Some(p) => FormatConfig::load(p),
        None => FormatConfig::new(),
    };

    // Run processing.
    let input = read_content(args.in_path).expect("Failed to read input file");

    let parsed = parse(input).expect("Failed to parse content");

    let output = compose(parsed, &config).expect("Failed to compose output");

    save_content(args.out_path, output).expect("Failed to save to output file");
}
