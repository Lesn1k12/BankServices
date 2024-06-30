use crate::modules::Product;
use actix_web::web;
use sqlx::PgPool;

pub async fn read_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<i32>,
) -> Result<web::Json<Product>, actix_web::Error> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
        .bind(&*product_id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error read product! -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?;

    Ok(web::Json(product))
}
