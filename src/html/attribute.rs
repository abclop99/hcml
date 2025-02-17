//! Attributes

use std::fmt::Display;

use itertools::Itertools;

use super::node::Node;

/// An HTML attribute
#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.value {
            write!(f, r#"{}="{}""#, self.name, value)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

/// A list of [`Attribute`]s
#[derive(Debug, Default)]
pub struct AttributeList {
    pub inner: Vec<Attribute>,
}

impl Node for AttributeList {}
impl Display for AttributeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner.iter().map(|attr| attr.to_string()).join(" ")
        )
    }
}

impl AttributeList {
    /// Push an attribute to the end of the list
    pub fn push(&mut self, attribute: Attribute) {
        self.inner.push(attribute)
    }
}
