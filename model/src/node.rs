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
    /// The node's parents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Node>>,
    /// The node's children.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
}

