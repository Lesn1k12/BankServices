use actix_web::web;
use sqlx::PgPool;
use crate::delete_order::{deserialize_to_orders, get_row_from_orders};
use crate::modules::Order;

pub async fn read_order(pool: web::Data<PgPool>, order_id: web::Path<i32>) -> Result<web::Json<Vec<Order>>, actix_web::Error>{
    let row = get_row_from_orders(pool.clone(), &order_id).await?;
    Ok(web::Json(deserialize_to_orders(row)?))
}

