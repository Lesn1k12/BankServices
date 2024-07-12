use sqlx::{PgPool, Row};
use actix_web::web;
use sqlx::postgres::PgRow;
use crate::delete_order::{get_body_by_row, get_id_by_row};
use crate::modules::{Order, Orders};

pub async fn read_all_orders(pool: web::Data<PgPool>, client_id: web::Json<i32>) -> Result<web::Json<Vec<Orders>>, actix_web::Error>{
    let mut orders: Vec<Orders> = vec![];;
    let rows = get_rows(&pool).await?;

    for row in rows{
        let mut new_order = Orders::new();
        let user_id = get_user_id_by_row(&row)?;
        if user_id == *client_id {
            Orders::update(&mut new_order, get_id_by_row(&row)?, user_id, get_user_name_by_row(&row)?, get_body_by_row(&row)?);
            orders.push(new_order)
        }
    }

    Ok(web::Json(orders))
}

async fn get_rows(pool: &web::Data<PgPool>) -> Result<Vec<PgRow>, actix_web::Error>{
    sqlx::query("SELECT * FROM orders")
        .fetch_all(pool.get_ref())
        .await.map_err(|e|{
        log::error!("Error to get rows! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error to get rows! Maybe order list is empty! -> {}", e))
    })
}

pub fn get_user_id_by_row(row: &PgRow) -> Result<i32, actix_web::Error>{
    row.try_get("user_id").map_err(|e| {
        log::error!("Error to get user_ID from orders! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Error to get user_ID from orders! -> {}",
            e
        ))
    })
}

pub fn get_user_name_by_row(row: &PgRow) -> Result<String, actix_web::Error>{
    row.try_get("user_name").map_err(|e| {
        log::error!("Error to get ID from orders! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Error to get ID from orders! -> {}",
            e
        ))
    })
}