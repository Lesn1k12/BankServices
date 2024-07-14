// маршрутизація

use crate::handlers::{auth_handler, orders_handler, products_handler};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/login").route(web::post().to(auth_handler::login)))
            .service(web::resource("/register").route(web::post().to(auth_handler::register)))
            .service(web::resource("/index").route(web::get().to(auth_handler::index)))
            .service(
                web::resource("/create_product")
                    .route(web::post().to(products_handler::handler_create_product)),
            )
            .service(
                web::resource("/read_all_products")
                    .route(web::post().to(products_handler::handler_sort_product)),
            )
            .service(
                web::resource("/read_product/{id}")
                    .route(web::get().to(products_handler::handler_read_product)),
            )
            .service(
                web::resource("/remove_product/{id}")
                    .route(web::delete().to(products_handler::handler_remove_product)),
            )
            .service(
                web::resource("/update_product/{id}")
                    .route(web::put().to(products_handler::handler_update_product)),
            )
            .service(
                web::resource("/create_order")
                    .route(web::post().to(orders_handler::order_create_handler)),
            )
            .service(
                web::resource("/delete_order/{id}")
                    .route(web::delete().to(orders_handler::order_delete_handler)),
            )
            .service(
                web::resource("/read_order/{id}")
                    .route(web::get().to(orders_handler::read_order_handler)),
            )
            .service(
                web::resource("/read_all_orders")
                    .route(web::post().to(orders_handler::read_all_orders_handler)),
            ),
    );
}
