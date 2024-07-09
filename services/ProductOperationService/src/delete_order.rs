use crate::modules::{Order, Orders, Product, ProductUpdate};
use crate::update_product::update_product;
use actix_web::web::Query;
use actix_web::{web, HttpResponse};
use sqlx::postgres::PgRow;
use sqlx::PgPool;
use sqlx::Row;

pub async fn delete_order(
    pool: web::Data<PgPool>,
    orders_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let row = get_row_from_orders(pool.clone(), &orders_id).await?;
    let orders = deserialize_to_orders(row)?;

    for order in orders {
        if let Some(order_id) = order.product.id {
            let product_update = ProductUpdate {
                storage_quantity: Some(&order.product.storage_quantity + &order.wanted_quantity),
                ..Default::default()
            };
            update_product(
                pool.clone(),
                web::Path::from(order_id),
                web::Json(product_update),
            )
                .await?;
        }
    }

    delete_from_orders_table(pool.clone(), orders_id).await?;

    Ok(HttpResponse::Ok().json("Order deleted successfully!"))
}

async fn get_row_from_orders(
    pool: web::Data<PgPool>,
    orders_id: &web::Path<i32>,
) -> Result<PgRow, actix_web::Error> {
    sqlx::query(
        "SELECT orders
        FROM orders
        WHERE id = $1",
    )
        .bind(&**orders_id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error getting order for deletion: {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Error getting order for deletion: {}",
                e
            ))
        })
}

async fn delete_from_orders_table(
    pool: web::Data<PgPool>,
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

fn deserialize_to_orders(row: PgRow) -> Result<Vec<Order>, actix_web::Error> {
    let order_json: serde_json::Value = row.try_get("orders").map_err(|e| {
        log::error!("Error to get row from orders! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Error to get row from orders! -> {}",
            e
        ))
    })?;

    serde_json::from_value(order_json.clone()).map_err(|e| {
        log::error!("Error parsing order JSON: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error parsing order JSON: {}", e))
    })
}
