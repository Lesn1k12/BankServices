use crate::create_product::create_product;
use crate::read_all_products::read_all_products;
use crate::read_product::read_product;
use crate::remove_product::remove_product;
use crate::update_product::update_product;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn configure_product(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("products")
            .route("", web::post().to(create_product))
            .route("", web::get().to(read_all_products))
            .route("/{id}", web::get().to(read_product))
            .route("/{id}", web::delete().to(remove_product))
            .route("/{id}", web::put().to(update_product)),
    );
}
