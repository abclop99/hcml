//! Handles HBML tags

use pest::iterators::{Pair, Pairs};
use smart_default::SmartDefault;

use crate::{
    html::{
        attribute::{Attribute, AttributeList},
        element::Element,
    },
    parser::{string::create_string, Rule},
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

                // Get inner of attribute
                let mut inner = pair.into_inner();

                // Get name of attribute (expect ident)
                let name = inner.next().expect("Missing attr name: {pair:?}");
                debug_assert_eq!(name.as_rule(), Rule::ident);
                let name = name.as_str().to_owned();

                // Get value of attribute (Optional String)
                let value = inner.next().map(|p| create_string(p.into_inner()));

                // Ensure no other pairs
                debug_assert_eq!(
                    inner.next(),
                    None,
                    "Unexpected pair after `value` in attribute {inner:?}"
                );

                let attribute = Attribute { name, value };

                tag.attributes.push(attribute);
            }
            rule => panic!("Unexpected rule `{rule:?}` in `tag`: {pair:?}"),
        }
    }

    tag
}
