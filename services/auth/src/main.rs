mod db;
mod handlers;
mod models;
mod schema;
mod tests_auth;

use actix_web::{web, HttpServer, App};
use crate::db::init_pool;
use crate::handlers::init;
use env_logger::Env;
use log::info;
use actix_cors::Cors;
use actix_web::http::header;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting auth service at http://127.0.0.1:8081");

    let pool = init_pool().expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(init)
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
