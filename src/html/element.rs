//! Elements

use std::fmt::{self};

use smart_default::SmartDefault;

use super::{
    attribute::{Attribute, AttributeList},
    node::{Node, NodeList},
};

#[derive(Debug, SmartDefault)]
pub struct Element {
    /// The name of the element
    ///
    /// # Example:
    ///
    /// The following element has the name `div`
    ///
    /// ```html
    /// <div></div>
    /// ```
    #[default = "div"]
    pub name: String,
    pub attributes: AttributeList,
    pub children: NodeList,

    /// Whether the element is a void element
    pub void: bool,
}

impl Element {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn with_attributes(self, attributes: AttributeList) -> Self {
        Self { attributes, ..self }
    }

    pub fn add_attribute(self, attribute: Attribute) -> Self {
        let mut attributes = self.attributes;
        attributes.push(attribute);

        Self { attributes, ..self }
    }

    pub fn with_children(self, children: NodeList) -> Self {
        Self { children, ..self }
    }

    pub fn add_child(self, child: Box<dyn Node>) -> Self {
        let mut children = self.children;
        children.push(child);

        Self { children, ..self }
    }

    pub fn with_void(self, void: bool) -> Self {
        Self { void, ..self }
    }
}

impl Node for Element {}
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.void {
            debug_assert!(
                self.children.is_empty(),
                "Void nodes can't have any children"
            );
            write!(f, "<{0} {1}>", self.name, self.attributes)
        } else {
            // Surround children string with spaces
            let children = format!(" {} ", self.children);

            write!(
                f,
                "<{0} {1}>{2}</{0}>",
                self.name, self.attributes, children
            )
        }
    }
}
