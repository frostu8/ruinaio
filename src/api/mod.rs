//! Ruina REST API.

pub mod node;

use actix_web::web;

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

