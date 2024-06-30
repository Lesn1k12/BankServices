use actix_web::{HttpResponse, Responder, web, http::StatusCode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use log::info;
use std::env;

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


pub async fn create_product(client: web::Data<Client>, req: web::Json<Product>) -> impl Responder {
    let products_service_url = "http://localhost:8083";
    let url = format!("{}/products/create_product/", products_service_url);

    let res = client.post(&url)
        .json(&*req)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn read_all_products(client: web::Data<Client>) -> impl Responder {
    let products_service_url = "http://localhost:8083";
    let url = format!("{}/products/read_all_products/", products_service_url);

    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            info!("Received response with status: {}", status);
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        },
        Err(e) => {
            info!("Error connecting: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn read_product(
    client: web::Data<Client>,
    product_id: web::Path<i32>,
) -> impl Responder {
    let products_service_url = "http://localhost:8083";
    let url = format!("{}/products/read_product/{}", products_service_url, product_id);

    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn remove_product(client: web::Data<Client>, product_id: web::Path<i32>) -> impl Responder {
    let products_service_url = "http://localhost:8083";
    let url = format!("{}/products/remove_product/{}", products_service_url, product_id);

    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_product(client: web::Data<Client>, product_id: web::Path<i32>) -> impl Responder {
    let products_service_url = "http://localhost:8083";
    let url = format!("{}/products/update_product/{}", products_service_url, product_id);

    let res = client.put(&url).send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

