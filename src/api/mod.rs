//! Ruina REST API.

pub mod node;

use actix_web::web;

/// Configures an actix web application with the API.
pub fn config(app: &mut web::ServiceConfig) {
    app.service(web::resource("/nodes")
            .route(web::get().to(node::list))
        );
}

