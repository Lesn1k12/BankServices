use sqlx::PgPool;
use actix_web::web;
use crate::delete_order::deserialize_to_orders;
use crate::modules::Order;

pub async fn read_all_orders(pool: web::Data<PgPool>, client_id: web::Path<i32>) -> Result<web::Json<Vec<Order>>, actix_web::Error>{
    let mut orders_to_sort = vec![];
    let mut result_order = vec![];

    let row_all_orders = sqlx::query(
        "SELECT orders FROM orders",
    )
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error getting order for deletion: {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Error getting order for deletion: {}",
                e
            ))
        })?;

    for orders in row_all_orders{
        let result = deserialize_to_orders(orders)?;
        orders_to_sort.push(result)
    }

    for all_orders in orders_to_sort{
        for sorting in all_orders{
            if sorting.user_id == *client_id{
                result_order.push(sorting)
            }
        }
    }

    Ok(web::Json(result_order))
}