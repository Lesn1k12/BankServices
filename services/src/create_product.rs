use crate::modules::{Product, ProductUpdate};
use actix_web::web;
use actix_web::HttpResponse;
use sqlx::PgPool;

pub async fn create_product(
    pool: web::Data<PgPool>,
    product: web::Json<Product>,
) -> Result<HttpResponse, actix_web::Error> {
    let _ = sqlx::query(
        r#"
        INSERT INTO products (name, price, category, storage_country, storage_region, storage_street, storage_quantity)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
        .bind(&product.name)
        .bind(&product.price)
        .bind(&product.category)
        .bind(&product.storage_country)
        .bind(&product.storage_region)
        .bind(&product.storage_street)
        .bind(&product.storage_quantity)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error to create product -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?;

    Ok(HttpResponse::Ok().json("Product created successfully"))
}
