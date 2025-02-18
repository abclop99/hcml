//! Handles HBML blocks

use either::Either;
use pest::iterators::{Pair, Pairs};

use crate::{
    html::{
        element::Element,
        node::{Node, NodeList},
    },
    parser::{create_node, element::create_element, string::create_string, Rule},
};

/// Handles a HBML block
///
/// ```pest
/// block =  { "{" ~ node_list ~ "}" }
/// node_list = _{ node* }
/// ```
///
/// # Parameters:
/// - pairs: The inner of the element pair
pub(crate) fn create_block(inner: Pairs<Rule>) -> NodeList {
    let mut node_list = NodeList::new();

    for pair in inner {
        let node = create_node(pair.into_inner());

        // Box the node if necessary
        let node = match node {
            Either::Left(node) => node,
            Either::Right(node_list) => Box::new(node_list),
        };

        node_list.push(node);
    }

    node_list
}
