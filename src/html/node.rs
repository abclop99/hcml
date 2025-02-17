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
    pub fn push(&mut self, node: Box<dyn Node>) {
        self.inner.push(node)
    }
}
