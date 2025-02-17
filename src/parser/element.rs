//! Handles HBML elements

use pest::iterators::{Pair, Pairs};
use tag::extract_tag;

use crate::{html::element::Element, parser::Rule};

pub mod tag;

/// Handles a HBML element
///
/// # Grammar for element:
///
/// ```pest
/// element = { tag ~ (node | void) }
/// ```
///
/// # Parameters:
/// - pairs: The inner of the comment pair
pub(crate) fn create_element(mut inner: Pairs<Rule>) -> Element {
    debug_assert_eq!(
        inner.len(),
        2,
        "Expected 2 pairs in element. Found {}: {:?}",
        inner.len(),
        inner
    );

    // Get tag
    let tag = inner.next().expect("Tag missing");
    debug_assert_eq!(
        tag.as_rule(),
        Rule::tag,
        "Unexpected rule {:?} found. Expected `tag`: {:?}",
        tag.as_rule(),
        tag
    );

    let tag = extract_tag(tag.into_inner());

    dbg!(tag);

    // Get content
    let content = inner.next().expect("Content");

    match content.as_rule() {
        Rule::node => todo!("element > node"),
        Rule::void => todo!("element > void"),
        rule => panic!("Unexpected content rule {rule:?}: {content:?}"),
    }

    todo!("create_element")
}
