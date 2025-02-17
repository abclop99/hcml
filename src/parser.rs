//! For parsing HCML

use node::create_node;
use pest::{error::Error as PestError, iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

pub mod comment;
pub mod element;
pub mod node;
pub mod string;
use comment::create_comment;

use crate::html::node::Node;

/// Parses a HCML document/partial into an HTML DOM
pub fn parse(document: &str) -> Result<String, HcmlError> {
    // Parse document
    let pairs = HcmlParser::parse(Rule::document, document)?;

    // Stuff with pairs
    for pair in pairs {
        let _thing = parse_document(pair);
    }

    todo!("Return thing")
}

// Handles one element or comment in the document
fn parse_document(pair: Pair<Rule>) -> Box<dyn Node> {
    use Rule::*;

    match pair.as_rule() {
        EOI => todo!("EOI"),
        COMMENT => Box::new(create_comment(pair.into_inner())), //comment::handle_comment(pair.into_inner()),
        node => create_node(pair.into_inner()),
        rule => {
            unimplemented!("Unexpected {rule:?} in document: {pair:?}")
        }
    }
}

/// HCML parser
#[derive(Parser)]
#[grammar = "hcml.pest"]
pub(crate) struct HcmlParser;

#[derive(Error, Debug)]
pub enum HcmlError {
    #[error("Error parsing document: {0}")]
    ParseError(#[from] PestError<Rule>),
}
