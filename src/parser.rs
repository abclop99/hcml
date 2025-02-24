//! For parsing HCML

use node::create_node;
use pest::{error::Error as PestError, iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

pub mod block;
pub mod comment;
pub mod element;
pub mod node;
pub mod string;
use comment::create_comment;

use crate::html::{
    node::{Node, NodeList},
    null::Null,
};

/// Parses a HCML document/partial into an HTML DOM
pub fn parse(document: &str) -> Result<NodeList, HcmlError> {
    // Parse document
    let pairs = HcmlParser::parse(Rule::document, document)?;

    let mut node_list = NodeList::new();

    // Stuff with pairs
    for pair in pairs {
        node_list.push(parse_document(pair));
    }

    Ok(node_list)
}

// Handles one element or comment in the document
fn parse_document(pair: Pair<Rule>) -> Box<dyn Node> {
    use Rule::*;

    match pair.as_rule() {
        EOI => Box::new(Null),
        COMMENT => Box::new(create_comment(pair.into_inner())),
        node => create_node(pair.into_inner()).either(|n| n, |b| Box::new(b)),
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
    ParseError(#[from] Box<PestError<Rule>>),
}

// Box the [`PestError`] so other [`HcmlError`]s can be smaller
impl From<PestError<Rule>> for HcmlError {
    fn from(value: PestError<Rule>) -> Self {
        Self::ParseError(Box::new(value))
    }
}
