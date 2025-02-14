use pest::Parser;

use hcml;

fn main() {
    let document = include_str!("../test/example-1.hcml");

    let result = hcml::parse(document);

    match result {
        Ok(pairs) => println!("Parsed successfully: {pairs:?}"),
        Err(e) => println!("Parse failed: {e}"),
    }
}
