#[macro_use]
extern crate log;

use actix_web::{App, HttpServer, web};

use std::env;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    info!("establishing connection to database");

    let database_url = env::var("DATABASE_URL")?;
    let database = sqlx::PgPool::connect(&database_url).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .configure(ruinaio::api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(From::from)
}

