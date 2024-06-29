use crate::create_product::create_product;
use crate::read_all_products::read_all_products;
use crate::read_product::read_product;
use crate::remove_product::remove_product;
use crate::update_product::update_product;
use actix_web::web;
use actix_web::web::ServiceConfig;

fn configure_product(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("/create_product", web::post().to(create_product))
            .route("/read_all_products)", web::get().to(read_all_products))
            .route("/read_product/{id}", web::get().to(read_product))
            .route("/remove_product/{id}", web::delete().to(remove_product))
            .route("/update_product/{id}", web::put().to(update_product)),
    );
}

