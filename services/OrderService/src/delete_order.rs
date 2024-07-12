use crate::modules::{Order, Orders, Product, ProductUpdate};
use crate::update_product::update_product;
use actix_web::web::Query;
use actix_web::{web, HttpResponse};
use sqlx::postgres::PgRow;
use sqlx::PgPool;
use sqlx::Row;
use serde_json::Value;

pub async fn delete_order(
    pool: web::Data<PgPool>,
    orders_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let row = get_row(&pool, &orders_id).await?;
    let id = get_id_by_row(&row)?;
    let body = get_body_by_row(&row)?;

    for mut order in body{
        order.product_storage_quantity += order.wanted_quantity;
        let product_update = ProductUpdate {
            storage_quantity: Some(order.product_storage_quantity),
            ..Default::default()
        };

        update_product(pool.clone(), web::Path::from(order.product_id), web::Json(product_update)).await?;
        log::info!("Product updated successfully!");
    }

    delete_from_orders_table(&pool, web::Path::from(id)).await?;

    Ok(HttpResponse::Ok().json("Order deleted successfully!"))
}

pub async fn get_row(pool: &web::Data<PgPool>, order_id: &web::Path<i32>) -> Result<PgRow, actix_web::Error>{
     sqlx::query("SELECT * FROM orders WHERE id = $1")
        .bind(&**order_id)
        .fetch_one(pool.get_ref())
        .await.map_err(|e|{
            log::error!("Error to get row by id! -> {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error to get row by id!. Maybe id is not found, or list is empty!!! -> {}", e))
        })
}

async fn delete_from_orders_table(
    pool: &web::Data<PgPool>,
    orders_id: web::Path<i32>,
) -> Result<(), actix_web::Error> {
    sqlx::query("DELETE FROM orders WHERE id = $1")
        .bind(*orders_id)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error deleting order: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error deleting order: {}", e))
        })?;

    Ok(())
}

pub fn get_id_by_row(row: &PgRow) -> Result<i32, actix_web::Error>{
    row.try_get("id").map_err(|e| {
        log::error!("Error to get ID from orders! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Error to get ID from orders! -> {}",
            e
        ))
    })
}


pub fn get_body_by_row(row: &PgRow) -> Result<Vec<Order>, actix_web::Error>{
    let orders: Value = row.try_get("orders").map_err(|e| {
        log::error!("Error to get orders from orders! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Error to get orders from orders! -> {}",
            e
        ))
    })?;

    serde_json::from_value(orders).map_err(|e| {
        log::error!("Error parsing order JSON -> {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error parsing order JSON -> {}", e))
    })
}


