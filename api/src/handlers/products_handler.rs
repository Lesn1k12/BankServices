use crate::models::products::{Product, WantedSortItem};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use log::info;
use reqwest::{Client, Response};
use serde_json::Value;

const PRODUCT_SERVICE_URL: &str = "http://localhost:8083";

pub async fn handler_create_product(
    client: web::Data<Client>,
    product: web::Json<Product>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/products/create_product", PRODUCT_SERVICE_URL);

    let response = send_request(client.post(&url), Some(&product)).await?;
    build_response(response).await
}

pub async fn handler_read_product(
    client: web::Data<Client>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!(
        "{}/products/read_product/{}",
        PRODUCT_SERVICE_URL, product_id
    );

    let response = send_request(client.get(&url), None::<&Value>).await?;
    build_response(response).await
}

pub async fn handler_remove_product(
    client: web::Data<Client>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!(
        "{}/products/remove_product/{}",
        PRODUCT_SERVICE_URL, product_id
    );

    let response = send_request(client.delete(&url), None::<&Value>).await?;
    build_response(response).await
}

pub async fn handler_update_product(
    client: web::Data<Client>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!(
        "{}/products/update_product/{}",
        PRODUCT_SERVICE_URL, product_id
    );

    let response = send_request(client.put(&url), None::<&Value>).await?;
    build_response(response).await
}

pub async fn handler_sort_product(
    client: web::Data<Client>,
    wanted_sort_item: Option<web::Json<WantedSortItem>>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/products/sort_products", PRODUCT_SERVICE_URL);
    info!("hihihaha");

    let response = if let Some(wanted_sort_item) = wanted_sort_item {
        send_request(client.post(&url), Some(&wanted_sort_item)).await?
    } else {
        send_request(client.post(&url), None::<&Value>).await?
    };

    build_response(response).await
}

pub async fn build_response(response: Response) -> Result<HttpResponse, actix_web::Error> {
    let status = response.status();
    let response_body = get_response_body(response).await?;
    let converted_status = convert_status_code_to_u16(&status)?;

    Ok(HttpResponse::build(converted_status).body(response_body))
}

pub async fn get_response_body(response: Response) -> Result<String, actix_web::Error> {
    response.text().await.map_err(|e| {
        log::error!("Error to get response body -> {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error to get response body -> {}", e))
    })
}

fn convert_status_code_to_u16(
    status: &reqwest::StatusCode,
) -> Result<StatusCode, actix_web::Error> {
    StatusCode::from_u16(status.as_u16()).map_err(|e| {
        log::error!("Error to convert response status to u16 -> {}", e);
        actix_web::error::ErrorInternalServerError(format!(
            "Error to convert response status to u16 -> {}",
            e
        ))
    })
}

pub async fn send_request<T>(
    client: reqwest::RequestBuilder,
    body: Option<&T>,
) -> Result<Response, actix_web::Error>
where
    T: serde::Serialize,
{
    if let Some(body) = body {
        client.json(body).send().await.map_err(|e| {
            log::error!("Error to send request ! -> {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error to send request ! -> {}", e))
        })
    } else {
        client.send().await.map_err(|e| {
            log::error!("Error to send request ! -> {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error to send request ! -> {}", e))
        })
    }
}
