//! Node API.

use ruinaio_model::{params, node::Node, slug};

use crate::db::Db;
use crate::error::{Code, Error};

use std::borrow::Cow;

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
        "SELECT id, slug, title, body FROM node LIMIT $1 OFFSET $2;"
    )
        .bind(limit)
        .bind(offset)
        .try_map(|row| Ok(Node {
            id: row.try_get(0)?,
            slug: row.try_get(1)?,
            title: row.try_get(2)?,
            body: row.try_get(3)?,
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
    let params::CreateNode { namespace, title, body } = params.into_inner();

    let namespace = match namespace {
        Some(namespace) if namespace.is_empty() => None,
        namespace => namespace,
    };

    // create a slug
    let slug = check_title(&title)?;
    let slug = match namespace {
        Some(namespace) => {
            let namespace = check_namespace(&namespace)?.to_owned();
            (namespace + &slug).into()
        }
        None => slug,
    };

    // create new node
    let (id,) = sqlx::query_as::<_, (i32,)>(
        "INSERT INTO node (slug, title, body) VALUES ($1, $2, $3) RETURNING id;"
    )
        .bind(&slug)
        .bind(&title)
        .bind(&body)
        .fetch_one(db.get_ref())
        .await?;

    // return node
    Ok(web::Json(Node {
        id,
        slug: slug.into_owned(),
        title,
        body,
    }))
}

/// Gets a single node with all of its children and parents.
pub async fn node(
    id: web::Path<(i32,)>,
    db: Db,
) -> Result<web::Json<Node>, Error> {
    let (id,) = id.into_inner();
    
    // fetch node
    let node = sqlx::query_as::<_, (String, String, String)>(
        "SELECT slug, title, body FROM node WHERE id = $1;"
    )
        .bind(id)
        .fetch_optional(db.get_ref())
        .await?;

    if let Some((slug, title, body)) = node {
        Ok(web::Json(Node { id, slug, title, body }))
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
    let params::UpdateNode { namespace, title, body } = params.into_inner();

    let namespace = match namespace {
        Some(Some(namespace)) if namespace.is_empty() => Some(None),
        namespace => namespace,
    };

    // create slug
    let slug = match (namespace, &title) {
        // updates both the namespace and title, effectively giving it an
        // entirely new slug
        (Some(Some(namespace)), Some(title)) => {
            Some(check_namespace(&namespace)?.to_owned() + &check_title(title)?)
        }
        // unsets the namespace and updates the slug
        (Some(None), Some(title)) => {
            let title = check_title(title)?;

            Some(title.into_owned())
        }
        // updates only the namespace
        (Some(Some(namespace)), None) => {
            let namespace = check_namespace(&namespace)?;

            let slug = get_slug(id, &db).await?;
            let (_, title) = slug::split(&slug);

            Some(namespace.to_owned() + title)
        }
        // unsets the namespace
        (Some(None), None) => {
            let slug = get_slug(id, &db).await?;

            let (_, title) = slug::split(&slug);

            Some(title.to_owned())
        }
        // updates only the slug
        (None, Some(title)) => {
            let title = check_title(title)?;

            let slug = get_slug(id, &db).await?;
            let (namespace, _) = slug::split(&slug);

            match namespace {
                Some(namespace) => Some(namespace.to_owned() + &title),
                None => Some(title.into_owned()),
            }
        }
        // updates nothing!
        (None, None) => None
    };

    // update node in database
    let node = sqlx::query_as::<_, (String, String, String)>(
        "UPDATE node SET slug = COALESCE($2, slug), title = COALESCE($3, title), body = COALESCE($4, body) WHERE id = $1 RETURNING slug, title, body"
    )
        .bind(id)
        .bind(slug)
        .bind(title)
        .bind(body)
        .fetch_optional(db.get_ref())
        .await?;

    // retrieve node
    if let Some((slug, title, body)) = node {
        Ok(web::Json(Node { id, slug, title, body }))
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

async fn get_slug(id: i32, db: &Db) -> Result<String, Error> {
    sqlx::query_as::<_, (String,)>("SELECT slug FROM node WHERE id = $1")
        .bind(id)
        .fetch_one(db.get_ref())
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => Error::not_found("node not found"),
            err => Error::from(err),
        })
        .map(|(s,)| s)
}

fn check_title<'a>(s: &'a str) -> Result<Cow<'a, str>, Error> {
    if s.len() == 0 {
        return Err(Error::payload_too_large("member `title` must be at least 1 character or more"));
    }

    if s.len() > 128 {
        return Err(Error::payload_too_large("member `title` must be less than or equal to 128 characters"));
    }

    // if title is less than 128 characters, the slug should be, too.
    ruinaio_model::slug::slugify(s).map_err(Into::into)
}

fn check_namespace<'a>(s: &'a str) -> Result<&'a str, Error> {
    if s.len() == 0 {
        return Err(Error::payload_too_large("member `namespace` must be at least 1 character or more"));
    }

    if s.len() > 128 {
        return Err(Error::payload_too_large("member `namespace` must be less than or equal to 128 characters"));
    }

    if s.chars().last().unwrap() != '/' {
        return Err(Error::payload_too_large("member `namespace` must end in a slash"));
    }

    ruinaio_model::slug::check_slug(s)
        .map_err(|err| Error::new(Code::InvalidSlug, err.to_string()))
}

