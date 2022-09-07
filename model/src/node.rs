use serde::{Deserialize, Serialize};

/// A single node.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    /// The unique identifier of the node.
    pub id: i32,
    /// The node's title.
    pub title: String,
    /// The actual content of the node.
    pub body: String,
    /// The node's parents.
    pub parents: Option<Vec<Node>>,
    /// The node's children.
    pub children: Option<Vec<Node>>,
}

/// Request query parameters for `GET /nodes`
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ParamsList {
    pub page: u32,
    pub limit: u32,
}

impl Default for ParamsList {
    fn default() -> ParamsList {
        ParamsList {
            page: 1,
            limit: 20,
        }
    }
}

