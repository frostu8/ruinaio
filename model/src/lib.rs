//! Ruina's data model.

pub mod error;
pub mod node;
pub mod params;
pub mod slug;
mod patch;

pub use node::Node;
pub use error::Error;
pub use patch::Patch;

