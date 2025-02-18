//! A program to convert HCML to HTML.

use std::{fs, process};

use clap::Parser;

use hcml;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    let document = match fs::read_to_string(args.file) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            process::exit(1)
        }
    };

    let result = hcml::parser::parse(&document);

    match result {
        Ok(html) => {
            eprintln!("Parsed successfully:");
            println!("{html}")
        }
        Err(e) => eprintln!("Parse failed: {e}"),
    }
}
