//! A program to convert HCML to HTML.

use hcml;

fn main() {
    let document = include_str!("../test/example-1.hcml");

    let result = hcml::parser::parse(document);

    match result {
        Ok(html) => {
            eprintln!("Parsed successfully:");
            println!("{html}")
        }
        Err(e) => eprintln!("Parse failed: {e}"),
    }
}
