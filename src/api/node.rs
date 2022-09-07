//! Node API.

use ruinaio_model::node::{Node, ParamsList};

use actix_web::{web, Responder};

/// Lists all the nodes in a space.
pub async fn list(
    params: web::Query<ParamsList>,
) -> web::Json<Vec<Node>> {
    // check bounds
    if params.page == 0 {
        // TODO: replace with proper error
        panic!("page must be greater than zero");
    }

    if params.limit > 20 {
        // TODO: replace with proper error
        panic!("limit cannot be greater than 20");
    }

    todo!()
}

