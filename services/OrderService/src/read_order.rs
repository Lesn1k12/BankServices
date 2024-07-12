use actix_web::web;
use sqlx::PgPool;
use crate::delete_order::{get_body_by_row, get_row};
use crate::modules::{Orders};
use crate::read_all_orders::{get_user_id_by_row, get_user_name_by_row};

pub async fn read_order(pool: web::Data<PgPool>, order_id: web::Path<i32>) -> Result<web::Json<Orders>, actix_web::Error>{
    let mut order = Orders::new();
    let row = get_row(&pool, &order_id).await?;
    Orders::update(&mut order, *order_id, get_user_id_by_row(&row)?, get_user_name_by_row(&row)?, get_body_by_row(&row)?);
    Ok(web::Json(order))
}

