//! Handles HBML comments

use pest::iterators::{Pair, Pairs};

use crate::{html::comment::Comment, parser::Rule};

/// Handles a HBML comment
///
/// # Structure of Comment in pest grammar:
///
/// ```text
/// COMMENT > hlc_text: "text"
/// ```
///
/// # Parameters:
/// - pairs: The inner of the comment pair
pub(crate) fn create_comment(inner: Pairs<Rule>) -> Comment {
    debug_assert_eq!(inner.len(), 1, "Only one comment type rule expected");

    // Get first contained pair
    let pair = inner.peek().expect("Must contain a pair");
    debug_assert!(
        [Rule::hlc_text, Rule::hbc_text].contains(&pair.as_rule()),
        "Unexpected rule {:?} found. Expected `hlc_text` or `hbc_text`.",
        pair.as_rule(),
    );

    // Get text
    let text = pair.as_span().as_str();

    // Create comment node
    Comment::new(text)
}
