mod logger;
mod server_config;
mod create_order;
mod modules;
mod update_product;
mod delete_order;
mod read_order;
mod read_all_orders;
mod test;


use actix_cors::Cors;
use actix_web::{web, web::ServiceConfig, HttpServer, Responder};
use logger::initial_logger;
use server_config::configure_order;
use sqlx::{postgres::PgPool, Pool, Postgres};

const DB_ADDRESS: &str = "postgres://postgres:7AIQF41SDJZ@localhost:5432/postgres";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    initial_logger();
    let pool = create_pool().await?;

    let _ = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_order)
    })
        .bind("127.0.0.1:8085")?
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
