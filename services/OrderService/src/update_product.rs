use crate::modules::ProductUpdate;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn update_product(
    pool: web::Data<PgPool>,
    product_id: web::Path<i32>,
    new_product: web::Json<ProductUpdate>,
) -> Result<HttpResponse, actix_web::Error> {
    let _ = sqlx::query(
        r#"
    UPDATE products
    SET name = COALESCE($1, name),
        price = COALESCE($2, price),
        category = COALESCE($3, category),
        storage_country = COALESCE($4, storage_country),
        storage_region = COALESCE($5, storage_region),
        storage_street = COALESCE($6, storage_street),
        storage_quantity = COALESCE($7, storage_quantity)
    WHERE id = $8
    "#,
    )
        .bind(&new_product.name)
        .bind(&new_product.price)
        .bind(&new_product.category)
        .bind(&new_product.storage_country)
        .bind(&new_product.storage_region)
        .bind(&new_product.storage_street)
        .bind(&new_product.storage_quantity)
        .bind(&*product_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error to update product -> {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error to update product -> {}", e))
        })?;

    Ok(HttpResponse::Ok().json("Product updated successfully"))
}
