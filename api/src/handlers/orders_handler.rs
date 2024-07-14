use crate::handlers::products_handler::{build_response, send_request};
use crate::models::orders::Orders;
use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde_json::Value;

const ORDER_SERVICE_URL: &str = "http://localhost:8085";
pub async fn order_create_handler(
    client: web::Data<Client>,
    order: web::Json<Orders>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/orders/create_order", ORDER_SERVICE_URL);
    let response = send_request(client.post(&url), Some(&order)).await?;
    build_response(response).await
}

pub async fn order_delete_handler(
    client: web::Data<Client>,
    order_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/orders/delete_order/{}", ORDER_SERVICE_URL, order_id);
    let response = send_request(client.delete(&url), None::<&Value>).await?;
    build_response(response).await
}

pub async fn read_all_orders_handler(
    client: web::Data<Client>,
    client_id: web::Json<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/orders/read_all_orders", ORDER_SERVICE_URL);
    let response = send_request(client.post(&url), Some(&client_id)).await?;
    build_response(response).await
}

pub async fn read_order_handler(
    client: web::Data<Client>,
    order_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/orders/read_order/{}", ORDER_SERVICE_URL, order_id);
    let response = send_request(client.post(&url), None::<&Value>).await?;
    build_response(response).await
}
