//! API parameters.

use serde::{Deserialize, Serialize};

/// Request query parameters for `GET /nodes`
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ListNodes {
    pub page: u32,
    pub limit: u32,
}

impl Default for ListNodes {
    fn default() -> ListNodes {
        ListNodes {
            page: 1,
            limit: 20,
        }
    }
}

/// Request body parameters for `PATCH /node/{node.id}`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// Request body parameters for `POST /node/new`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateNode {
    pub slug: String,
    pub body: String,
}

