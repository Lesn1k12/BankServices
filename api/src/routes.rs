// маршрутизація

use crate::handlers::auth_handler;
use actix_web::{get, web, HttpResponse};



pub fn init(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/login")
                    .route(web::post().to(auth_handler::login))
            )
            .service(
                web::resource("/register")
                    .route(web::post().to(auth_handler::register))
            )
            .service(
                web::resource("/index")
                    .route(web::get().to(auth_handler::index)
            )
        )
    );
}