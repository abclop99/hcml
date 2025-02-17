//! A program to convert HCML to HTML.

use hcml;

fn main() {
    let document = include_str!("../test/example-1.hcml");

    let result = hcml::parser::parse(document);

    match result {
        Ok(html) => println!("Parsed successfully: {html}"),
        Err(e) => println!("Parse failed: {e}"),
    }
}
