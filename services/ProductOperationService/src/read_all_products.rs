use crate::modules::Product;
use actix_web::web;
use sqlx::PgPool;

pub async fn read_all_products(
    pool: web::Data<PgPool>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let all_products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error to read all products -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?;
    Ok(web::Json(all_products))
}
