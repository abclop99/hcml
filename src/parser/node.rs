//! Handles HBML elements

use pest::iterators::{Pair, Pairs};

use crate::{
    html::{element::Element, node::Node},
    parser::{element::create_element, Rule},
};

/// Handles a HBML element
///
/// ```pest
/// node      = { element | string | lit_string | block }
/// ```
///
/// # Parameters:
/// - pairs: The inner of the element pair
pub(crate) fn create_node(inner: Pairs<Rule>) -> Box<dyn Node> {
    debug_assert_eq!(inner.len(), 1, "Only one pair expected in `node`");

    // Get first contained pair
    let pair = inner.peek().expect("Must contain a pair");

    match pair.as_rule() {
        Rule::element => Box::new(create_element(pair.into_inner())),
        Rule::string => todo!("node > string"),
        Rule::lit_string => todo!("node > lit_string"),
        Rule::block => todo!("node > block"),
        rule => panic!("Unexpected rule {rule:?} in node: {pair:?}"),
    }
}
