mod send_email_service;
mod logger;

use actix_cors::Cors;
use actix_web::{HttpServer, Responder};
use logger::initial_logger;
use send_email_service::send_email;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    initial_logger();

    let _ = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .max_age(3600),
            )
            .service(send_email)
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await;

    Ok(())
}
