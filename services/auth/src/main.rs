mod db;
mod handlers;
mod models;
mod schema;
mod tests;

use actix_web::{web, HttpServer, App, HttpResponse};
use anyhow::Result;
use dotenv::dotenv;
use crate::db::init_pool;
use crate::handlers::init;

use db::init_pool;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = init_pool()?;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(init)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    Ok(())
}



