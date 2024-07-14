use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::create_order::create_order;
use crate::delete_order::delete_order;
use crate::read_all_orders::read_all_orders;
use crate::read_order::read_order;

pub fn configure_order(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("/create_order", web::post().to(create_order))
            .route("/delete_order/{id}", web::delete().to(delete_order))
            .route("/read_order/{id}", web::get().to(read_order))
            .route("/read_all_orders", web::post().to(read_all_orders))
    );
}