use actix_web::{HttpResponse, Responder, web, http::StatusCode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use log::info;
use std::env;
use std::ops::Deref;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub price: f64,
    pub category: String,
    pub storage_country: String,
    pub storage_region: String,
    pub storage_street: String,
    pub storage_quantity: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUpdate {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub category: Option<String>,
    pub storage_country: Option<String>,
    pub storage_region: Option<String>,
    pub storage_street: Option<String>,
    pub storage_quantity: Option<i32>,
}

const PRODUCT_SERVICE_URL: &str = "http://localhost:8083";

pub async fn create_product(client: web::Data<Client>, reqwest: web::Json<Product>) -> impl Responder {
    let url = format!("{}/products/create_product/", PRODUCT_SERVICE_URL);
    let response = client.post(&url).json(&*reqwest).send().await;
    handle_response(response).await
}

pub async fn read_all_products(client: web::Data<Client>) -> impl Responder {
    let url = format!("{}/products/read_all_products/", PRODUCT_SERVICE_URL);
    let response = client.get(&url).send().await;
    handle_response(response).await
}

pub async fn read_product(client: web::Data<Client>, product_id: web::Path<i32>) -> impl Responder {
    let url = format!(
        "{}/products/read_product/{}",
        PRODUCT_SERVICE_URL, product_id
    );
    let response = client.get(&url).send().await;
    handle_response(response).await
}

pub async fn remove_product(
    client: web::Data<Client>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let url = format!(
        "{}/products/remove_product/{}",
        PRODUCT_SERVICE_URL, product_id
    );
    let response = client.get(&url).send().await;
    handle_response(response).await
}

pub async fn update_product(
    client: web::Data<Client>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let url = format!(
        "{}/products/update_product/{}",
        PRODUCT_SERVICE_URL, product_id
    );
    let response = client.put(&url).send().await;
    handle_response(response).await
}

async fn handle_response(response: Result<reqwest::Response, reqwest::Error>) -> impl Responder {
    match response {
        Ok(response) => {
            let status = response.status();
            let body = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    log::error!("Failed to read response body -> {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        }
        Err(e) => {
            log::error!("Failed to get response -> {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
