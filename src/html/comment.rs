//! Handles HTML comments

use std::fmt::{self, Display};

use super::node::Node;

#[derive(Debug, Clone)]
pub struct Comment {
    text: String,
}

impl Comment {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        let text = value.into();

        Self { text }
    }
}

impl Node for Comment {}
impl Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<!-- {} -->", self.text)
    }
}
