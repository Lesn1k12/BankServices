// маршрутизація

use actix_web::web;
use crate::handlers::auth_handler;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/login")
                    .route(web::post().to(auth_handler::login))
            )
    );
}