//! A null node

use std::fmt::Display;

use super::node::Node;

/// A null node that does not translate to anything.
#[derive(Debug, Default)]
pub struct Null;

impl Node for Null {}
impl Display for Null {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Does not write anything
        Ok(())
    }
}
