use crate::modules::{Order, Orders, ProductUpdate};
use crate::update_product::update_product;
use actix_web::{web, HttpResponse};
use core::default::Default;
use sqlx::types::Json;
use sqlx::PgPool;

async fn create_order(
    pool: web::Data<PgPool>,
    order_list: web::Json<Orders>,
) -> Result<HttpResponse, actix_web::Error> {
    for order_product in &order_list.orders {
        if let Some(product_id) = order_product.product.id {
            if order_product.product.storage_quantity >= order_product.wanted_quantity
                && order_product.product.storage_quantity > 0
            {
                update_to_product_table(&pool, product_id, order_product).await?;
                insert_to_order_table(&pool, order_product).await?;
            } else {
                return Err(actix_web::error::ErrorForbidden(
                    "We don't have that many items!",
                ));
            }
        } else {
            return Err(actix_web::error::ErrorBadRequest(
                "Я тебе говорил далбоебу отправить id вместе и продуктом!",
            ));
        }
    }

    Ok(HttpResponse::Ok().json("Order created successfully!"))
}

async fn update_to_product_table(pool: &web::Data<PgPool>, product_id: i32, order_product: &Order) -> Result<(), actix_web::Error>{
    let product_update = ProductUpdate {
        storage_quantity: Some(
            &order_product.product.storage_quantity - &order_product.wanted_quantity,
        ),
        ..Default::default()
    };

    update_product(
        pool.clone(),
        web::Path::from(product_id),
        web::Json(product_update),
    )
        .await?;

    Ok(())
}

async fn insert_to_order_table(pool: &web::Data<PgPool>, order_product: &Order) -> Result<(), actix_web::Error>{
    let _ = sqlx::query(
        r#"
                    INSERT INTO orders (product, wanted_quantity)
                    VALUES ($1, $2)
                    "#,
        )
        .bind(Json(&order_product.product))
        .bind(&order_product.wanted_quantity)
        .execute(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error to create order -> {}", e);
            actix_web::error::ErrorInternalServerError(format!(
                "Error to create order -> {}",
                e
            ))
        })?;

    Ok(())
}

