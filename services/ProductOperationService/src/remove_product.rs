use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use std::fmt::format;
pub async fn remove_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let _ = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(&*product_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error to remove product -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?;
    Ok(HttpResponse::Ok().json("Product removed successfully"))
}
