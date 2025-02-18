//! A HTML node representing anything that is a thing in HTML.

use std::fmt::{Debug, Display};

// pub type NodeList = Vec<Box<dyn Node>>;

pub trait Node: Debug + Display {}

#[derive(Debug, Default)]
pub struct NodeList {
    pub inner: Vec<Box<dyn Node>>,
}

impl Node for NodeList {}

impl Display for NodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.inner {
            Display::fmt(&**node, f)?;
        }

        Ok(())
    }
}

impl NodeList {
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self { inner: nodes }
    }

    // Pushes a node to the end of the list
    pub fn push(&mut self, node: Box<dyn Node>) {
        self.inner.push(node)
    }

    // Returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl From<Vec<Box<dyn Node>>> for NodeList {
    fn from(value: Vec<Box<dyn Node>>) -> Self {
        Self::new(value)
    }
}

impl Node for String {}
