use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hbml.pest"]
struct HbmlParser;

fn main() {
    let document = include_str!("../test/example-1.hbml");

    let result = HbmlParser::parse(Rule::document, document);

    match result {
        Ok(pairs) => println!("Parsed successfully: {pairs:?}"),
        Err(e) => println!("Parse failed: {e}"),
    }
}
