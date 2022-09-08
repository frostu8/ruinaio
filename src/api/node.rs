//! Node API.

use ruinaio_model::node::{Node, ParamsList};
use crate::db::Db;

use actix_web::{web, Responder};

use sqlx::Row as _;

/// Lists all the nodes in a space.
pub async fn list(
    params: web::Query<ParamsList>,
    db: Db,
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

    let limit = params.limit as i32;
    let offset = (params.page as i32 - 1) * limit;

    // return list of nodes
    sqlx::query(
        "SELECT id, title, body FROM node LIMIT $1 OFFSET $2;"
    )
        .bind(limit)
        .bind(offset)
        .try_map(|row| Ok(Node {
            id: row.try_get(0)?,
            title: row.try_get(1)?,
            body: row.try_get(2)?,
            parents: None,
            children: None,
        }))
        .fetch_all(db.get_ref())
        .await
        .map(|vec| web::Json(vec))
        // TODO: handle db errors
        .unwrap()
}

/// Gets a single node with all of its children and parents.
pub async fn node(
    id: web::Path<(i32,)>,
    db: Db,
) -> web::Json<Node> {
    let (id,) = id.into_inner();
    
    // fetch node
    let node = sqlx::query_as::<_, (String, String)>(
        "SELECT title, body FROM node WHERE id = $1;"
    )
        .bind(id)
        .fetch_optional(db.get_ref())
        .await
        // TODO: handle db errors
        .unwrap();

    match node {
        Some((title, body)) => {
            // get parents and children
            // TODO: handle db errors
            let parents = get_parents(id, &db).await.unwrap();
            let children = get_children(id, &db).await.unwrap();

            web::Json(Node { id, title, body, parents: Some(parents), children: Some(children) })
        }
        None => {
            // TODO: handle missing
            panic!("not found");
        }
    }
}

async fn get_parents(id: i32, db: &Db) -> Result<Vec<Node>, sqlx::Error> {
    sqlx::query(
        "SELECT node.id, node.title, node.body FROM node INNER JOIN relation ON node.id = relation.parent_id WHERE relation.child_id = $1"
    )
        .bind(id)
        .try_map(|row| Ok(Node {
            id: row.try_get(0)?,
            title: row.try_get(1)?,
            body: row.try_get(2)?,
            parents: None,
            children: None,
        }))
        .fetch_all(db.get_ref())
        .await
}

async fn get_children(id: i32, db: &Db) -> Result<Vec<Node>, sqlx::Error> {
    sqlx::query(
        "SELECT node.id, node.title, node.body FROM node INNER JOIN relation ON node.id = relation.child_id WHERE relation.parent_id = $1"
    )
        .bind(id)
        .try_map(|row| Ok(Node {
            id: row.try_get(0)?,
            title: row.try_get(1)?,
            body: row.try_get(2)?,
            parents: None,
            children: None,
        }))
        .fetch_all(db.get_ref())
        .await
}

