//! Error handling.

use std::ops::{Deref, DerefMut};
use std::fmt::{self, Display, Formatter};

use actix_web::{HttpResponse, ResponseError, body::BoxBody};
use actix_web::http::{StatusCode, header::ContentType};

pub use ruinaio_model::error::Code;

/// A web framework wrapper for a [`ruinaio_model::Error`].
#[derive(Clone, Debug)]
pub struct Error(pub ruinaio_model::Error);

impl Error {
    /// Creates a new `Error`.
    pub fn new<S>(code: Code, reason: S) -> Error
    where
        S: Into<String>,
    {
        Error(ruinaio_model::Error {
            code,
            reason: reason.into(),
        })
    }

    /// Creates a not found error with a specified message.
    pub fn not_found<S>(reason: S) -> Error
    where
        S: Into<String>,
    {
        Error::new(Code::NotFound, reason)
    }

    /// Creates an out of bounds error with a specified message.
    pub fn out_of_bounds<S>(reason: S) -> Error
    where
        S: Into<String>,
    {
        Error::new(Code::OutOfBounds, reason)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.code {
            Code::NotFound => StatusCode::NOT_FOUND,
            Code::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Code::InvalidSlug | Code::OutOfBounds => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .body(serde_json::to_string(&self.0).unwrap())
    }
}

impl<T> From<T> for Error
where
    T: std::error::Error
{
    fn from(error: T) -> Error {
        Error::new(Code::InternalServerError, error.to_string())
    }
}

impl From<Error> for ruinaio_model::Error {
    fn from(e: Error) -> ruinaio_model::Error {
        e.0
    }
}

impl Deref for Error {
    type Target = ruinaio_model::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

