//! Handles normal HBML strings

use pest::iterators::Pairs;

use crate::parser::Rule;

/// Handles a HBML string
///
/// # Grammar for element:
///
/// ```pest
/// string     = ${ "\"" ~ inner ~ "\"" }
/// ```
///
/// # Parameters:
/// - pairs: The inner of the comment pair
pub(crate) fn create_string(mut inner: Pairs<Rule>) -> String {
    let inner_pair = inner.next().expect("Expected `inner` in `string`");

    debug_assert_eq!(inner.next(), None, "Unexpected extra rule in string");

    debug_assert!(
        [Rule::inner, Rule::lit_inner].contains(&inner_pair.as_rule()),
        "Unexpected inner rule {:?}. Expected inner or lit_inner",
        inner_pair.as_rule()
    );

    inner_pair.as_str().to_owned()
}
