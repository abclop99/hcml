//! A program to convert HCML to HTML.

use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
    process::{self, exit},
};

use clap::Parser;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    // Get reader from file or stdin
    let reader: BufReader<Box<dyn Read>> = if args.file == PathBuf::from("-") {
        BufReader::new(Box::new(io::stdin()))
    } else {
        BufReader::new(Box::new(File::open(args.file).expect("Error opening file")))
    };

    // Read entire file
    let document = match io::read_to_string(reader) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            process::exit(1)
        }
    };

    let result = hcml::parser::parse(&document);

    match result {
        Ok(html) => {
            println!("{html}")
        }
        Err(e) => {
            eprintln!("Parse failed: {e}");
            exit(3);
        }
    }
}
