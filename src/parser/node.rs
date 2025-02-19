//! Handles HBML elements

use either::Either;
use pest::iterators::Pairs;

use crate::{
    html::node::{Node, NodeList},
    parser::{block::create_block, element::create_element, string::create_string, Rule},
};

/// Handles a HBML element
///
/// ```pest
/// node      = { element | string | lit_string | block }
/// ```
///
/// # Parameters:
/// - pairs: The inner of the element pair
pub(crate) fn create_node(inner: Pairs<Rule>) -> Either<Box<dyn Node>, NodeList> {
    debug_assert_eq!(inner.len(), 1, "Only one pair expected in `node`");

    // Get first contained pair
    let pair = inner.peek().expect("Must contain a pair");

    match pair.as_rule() {
        Rule::element => Either::Left(Box::new(create_element(pair.into_inner()))),
        Rule::string => Either::Left(Box::new(create_string(pair.into_inner()))),
        Rule::lit_string => Either::Left(Box::new(create_string(pair.into_inner()))),
        Rule::block => Either::Right(create_block(pair.into_inner())),
        rule => panic!("Unexpected rule {rule:?} in node: {pair:?}"),
    }
}
