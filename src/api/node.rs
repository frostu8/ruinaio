//! Node API.

use ruinaio_model::{params, node::Node};

use crate::db::Db;
use crate::error::Error;

use actix_web::{HttpResponse, web};

use sqlx::Row as _;

/// Lists all the nodes in a space.
pub async fn list(
    params: web::Query<params::ListNodes>,
    db: Db,
) -> Result<web::Json<Vec<Node>>, Error> {
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
        .map_err(From::from)
}

/// Creates a fresh node.
pub async fn create(
    params: web::Json<params::CreateNode>,
    db: Db,
) -> Result<web::Json<Node>, Error> {
    let params::CreateNode { title, body } = params.into_inner();

    if title.len() > 128 {
        return Err(Error::payload_too_large("member `title` must be less than or equal to 128 characters"));
    }

    // create new node
    let (id,) = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO node (title, body) VALUES ($1, $2) RETURNING id;"
    )
        .bind(&title)
        .bind(&body)
        .fetch_one(db.get_ref())
        .await?;

    // return node
    Ok(web::Json(Node {
        id, title, body,
        parents: None,
        children: None,
    }))
}

/// Gets a single node with all of its children and parents.
pub async fn node(
    id: web::Path<(i32,)>,
    db: Db,
) -> Result<web::Json<Node>, Error> {
    let (id,) = id.into_inner();
    
    // fetch node
    let node = sqlx::query_as::<_, (String, String)>(
        "SELECT title, body FROM node WHERE id = $1;"
    )
        .bind(id)
        .fetch_optional(db.get_ref())
        .await?;

    if let Some((title, body)) = node {
        // get parents and children
        let parents = get_parents(id, &db).await?;
        let children = get_children(id, &db).await?;

        Ok(web::Json(Node { id, title, body, parents: Some(parents), children: Some(children) }))
    } else {
        Err(Error::not_found("node not found"))
    }
}

/// Updates a single node.
pub async fn update(
    id: web::Path<(i32,)>,
    params: web::Json<params::UpdateNode>,
    db: Db,
) -> Result<web::Json<Node>, Error> {
    let (id,) = id.into_inner();
    let params::UpdateNode { title, body } = params.into_inner();

    if let Some(title) = &title {
        if title.len() > 128 {
            return Err(Error::payload_too_large("member `title` must be less than or equal to 128 characters"));
        }
    }

    // update node in database
    let node = sqlx::query_as::<_, (String, String)>(
        "UPDATE node SET title = $2, body = $3 WHERE id = $1 RETURNING title, body"
    )
        .bind(id)
        .bind(title)
        .bind(body)
        .fetch_optional(db.get_ref())
        .await?;

    // retrieve node
    if let Some((title, body)) = node {
        // get parents and children
        let parents = get_parents(id, &db).await?;
        let children = get_children(id, &db).await?;

        Ok(web::Json(Node { id, title, body, parents: Some(parents), children: Some(children) }))
    } else {
        Err(Error::not_found("node not found"))
    }
}

/// Delete a single node.
pub async fn delete(
    id: web::Path<(i32,)>,
    db: Db,
) -> Result<HttpResponse, Error> {
    let (id,) = id.into_inner();

    // delete node in database
    let result = sqlx::query(
        "DELETE FROM node WHERE id = $1"
    )
        .bind(id)
        .execute(db.get_ref())
        .await?;

    if result.rows_affected() > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(Error::not_found("node not found"))
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

