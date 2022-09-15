use serde::{Deserialize, Serialize};

/// A single node.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Node {
    /// The unique identifier of the node.
    pub id: i32,
    /// The node's unique slug. Unlike the `id`, this can change.
    pub slug: String,
    /// The node's title.
    pub title: String,
    /// The actual content of the node.
    pub body: String,
}

impl Node {
    /// Gets the namespace the node is in.
    pub fn namespace(&self) -> Option<&str> {
        let (namespace, _) = crate::slug::split(&self.slug);
        namespace
    }
}

