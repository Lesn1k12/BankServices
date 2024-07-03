mod create_product;
mod logger;
mod modules;
mod read_product;
mod remove_product;
mod server_config;
mod update_product;
mod sort_products;

use actix_cors::Cors;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpServer, Responder};
use create_product::create_product;
use logger::initial_logger;
use read_product::read_product;
use remove_product::remove_product;
use server_config::configure_product;
use sqlx::postgres::PgPool;
use sqlx::Pool;
use sqlx::Postgres;
use update_product::update_product;

const DB_ADDRESS: &str = "postgres://admin:admin@localhost:5432/bank_service";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    initial_logger();
    let pool = create_pool().await?;

    let _ = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .max_age(3600),
            )
            .app_data(pool.clone())
            .configure(configure_product)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await;

    Ok(())
}

async fn create_pool() -> anyhow::Result<Pool<Postgres>> {
    PgPool::connect(DB_ADDRESS).await.map_err(|e| {
        log::error!("Error to connect db! -> {}", e);
        anyhow::anyhow!("Error to connect db! -> {}", e)
    })
}
