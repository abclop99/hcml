//! HCML parser and translator (to HTML)

use pest::{error::Error as PestError, iterators::Pairs, Parser};
use pest_derive::Parser;

/// HCML parser
#[derive(Parser)]
#[grammar = "hcml.pest"]
struct HcmlParser;

type R = Rule;

/// Parses a HCML document/partial
pub fn parse(document: &str) -> Result<Pairs<'_, R>, PestError<R>> {
    HcmlParser::parse(Rule::document, document)
}
