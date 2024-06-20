mod routes;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::io::Result;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .configure(routes::init))
    })
    .bind("0.0.0.8080")?
    .run()
    .await
}
