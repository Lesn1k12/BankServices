mod routes;
mod handlers;
mod models;
mod tests;


use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use env_logger::Env;
use std::io::Result;
use actix_cors::Cors;
use actix_web::http::header;
use log::info;
use reqwest::Client;

#[actix_web::main] //api
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("api main");
    let client = Client::new();


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


