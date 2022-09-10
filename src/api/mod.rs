//! Ruina REST API.

pub mod node;

use actix_web::web;

use crate::error::{Code, Error};

/// Configures an actix web application with the API.
pub fn config(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/nodes")
            .route(web::get().to(node::list))
        )
        .service(web::resource("/nodes/new")
            .route(web::post().to(node::create))
        )
        .service(web::resource("/node/{id}")
            .route(web::get().to(node::node))
            .route(web::patch().to(node::update))
            .route(web::delete().to(node::delete))
        );
}

fn verify_slug<T>(s: T) -> Result<T, Error>
where
    T: AsRef<str>,
{
    // slugs must have:
    // - only ascii alphanumeric characters [A-Za-z0-9]
    // - preferably in PascalCase
    // - that's it

    // find the first character that doesn't follow this rule
    match s.as_ref().chars().enumerate().find(|(_, c)| !c.is_ascii_alphanumeric()) {
        Some((i, ch)) => {
            // create error
            Err(Error::new(Code::InvalidSlug, format!("character '{}' @ column {} in slug is invalid", ch, i+1)))
        }
        None => {
            // seems ok!
            Ok(s)
        }
    }
}

