use actix_web::{test, web, App};
use sqlx::{PgPool, Executor};
use serde_json::json;
use crate::modules::{Order, Orders};
use crate::create_order::create_order;
use crate::read_all_orders::read_all_orders;
use crate::logger::initial_logger;
// Ваши модели и функции

#[actix_rt::test]
async fn test_read_all_orders_order() {
    initial_logger();
    use actix_web::web::Json;
    let pool = PgPool::connect("postgres://postgres:7AIQF41SDJZ@localhost:5432/postgres").await.unwrap();
    use crate::delete_order::delete_order;

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/read_all_orders", web::post().to(read_all_orders))
    ).await;
    let client_id = Json(9);
    let req = test::TestRequest::post()
        .uri("/read_all_orders")
        .set_json(&client_id)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    let body = test::read_body(resp).await;
    let orders: Vec<Orders> = serde_json::from_slice(&body).unwrap();
    assert_eq!(orders.len(), 3)
}
