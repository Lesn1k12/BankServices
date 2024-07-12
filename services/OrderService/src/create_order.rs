use crate::modules::{Order, Orders, ProductUpdate};
use crate::update_product::update_product;
use actix_web::{web, HttpResponse};
use core::default::Default;
use sqlx::types::Json;
use sqlx::PgPool;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde_json::json;


pub async fn create_order(
    pool: web::Data<PgPool>,
    mut order_list: web::Json<Orders>,
) -> Result<HttpResponse, actix_web::Error> {
    for order in &mut order_list.orders{
        order.product_storage_quantity -= order.wanted_quantity;
        let product_update = ProductUpdate {
            storage_quantity: Some(order.product_storage_quantity),
            ..Default::default()
        };

        update_product(pool.clone(), web::Path::from(order.product_id), web::Json(product_update)).await?;
        log::info!("Product updated successfully!");
    }

    insert_to_order_table(&pool, &order_list).await?;
    log::info!("Order inserted successfully!");

    Ok(HttpResponse::Ok().json("Order created successfully!"))
}


async fn insert_to_order_table(
    pool: &web::Data<PgPool>,
    order_product: &Orders,
) -> Result<(), actix_web::Error> {
    let _ = sqlx::query(
        r#"
           INSERT INTO orders (orders, user_id, user_name)
           VALUES ($1, $2, $3)
        "#,
    )
        .bind(Json(&order_product.orders))
        .bind(&order_product.user_id)
        .bind(&order_product.user_name)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error to create order -> {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error to create order -> {}", e))
        })?;

    Ok(())
}
