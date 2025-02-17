//! Handles HBML tags

use pest::iterators::{Pair, Pairs};
use smart_default::SmartDefault;

use crate::{
    html::{
        attribute::{Attribute, AttributeList},
        element::Element,
    },
    parser::Rule,
};

#[derive(Debug, SmartDefault)]
pub struct Tag {
    #[default("div")]
    pub name: String,
    pub attributes: AttributeList,
}

/// Handles a HBML tag
///
/// # Grammar for tag:
///
/// ```pest
/// tag             =  {
///     ((name ~ s_id? | s_id | s_class) ~ (s_class)* | special_tag) ~ attributes?
/// }
/// ```
///
/// # Parameters:
/// - pairs: The inner of the comment pair
pub(crate) fn extract_tag(mut inner: Pairs<Rule>) -> Tag {
    eprintln!("{inner:?}");

    let mut tag = Tag::default();

    for pair in inner {
        match pair.as_rule() {
            Rule::name => tag.name = pair.as_str().to_owned(),
            Rule::id => tag.attributes.push(Attribute {
                name: "id".to_owned(),
                value: Some(pair.as_str().to_owned()),
            }),
            Rule::class => tag.attributes.push(Attribute {
                name: "class".to_owned(),
                value: Some(pair.as_str().to_owned()),
            }),
            Rule::attribute => {
                // Expect 2 pairs: ident & string for name & value
                dbg!(pair);
                todo!("tag > attribute")
            }
            rule => panic!("Unexpected rule `{rule:?}` in `tag`: {pair:?}"),
        }
    }

    dbg!(tag);

    todo!("extract_tag")
}
