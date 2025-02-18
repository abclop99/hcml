//! Handles HBML elements

use pest::iterators::{Pair, Pairs};
use tag::{extract_tag, Tag};

use crate::{
    html::{element::Element, node::NodeList},
    parser::{create_node, Rule},
};

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

    // Get content
    let content = inner.next().expect("Content");

    let (children, void) = match content.as_rule() {
        Rule::node => {
            let node = create_node(content.into_inner());

            (
                match node {
                    either::Either::Left(node) => vec![node].into(),
                    either::Either::Right(node_list) => node_list,
                },
                false,
            )
        }
        Rule::void => (NodeList::default(), true),
        rule => panic!("Unexpected content rule {rule:?}: {content:?}"),
    };

    let Tag { name, attributes } = tag;

    Element::new(name)
        .with_attributes(attributes)
        .with_children(children)
        .with_void(void)
}
