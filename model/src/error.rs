//! Ruina API errors.

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// An API error.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    /// A unique error code exactly describing the error.
    pub code: Code,
    /// A more human-readable reason as to why the error occured.
    pub reason: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{}: {}", self.code, self.reason)
    }
}

impl std::error::Error for Error { }

/// A unique identifier for an [`Error`].
#[repr(u32)]
#[derive(Clone, Copy, Debug, Deserialize_repr, Hash, Serialize_repr)]
pub enum Code {
    /// An internal server error occured. The message will contain basic debug
    /// information.
    InternalServerError = 21,
    /// Payload is too large for server to accept.
    PayloadTooLarge = 4001,
    /// The object was not found.
    NotFound = 4004,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

