//! Database accesses and management.

use sqlx::PgPool;

use actix_web::web;

/// The database type.
pub type Db = web::Data<PgPool>;

