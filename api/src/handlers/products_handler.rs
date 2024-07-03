use crate::modules::{Product, WantedAddress, WantedName, WantedPrice};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use reqwest::{Client, Response};
use serde_json::Value;

const PRODUCT_SERVICE_URL: &str = "http://localhost:8083";

pub async fn handler_create_product(
    client: web::Data<Client>,
    product: web::Json<Product>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/products/create_product", PRODUCT_SERVICE_URL);

    let response = client.post(&url).json(&product).send().await.map_err(|e| {
        log::error!("Error to get response with request! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error to get response -> {}", e))
    })?;

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
    
    let response = send_request(&client, &url, None::<&Value>).await?;
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

    let response = client.delete(&url).send().await.map_err(|e| {
        log::error!("Error to get response with request! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error to get response -> {}", e))
    })?;

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

    let response = client.put(&url).send().await.map_err(|e| {
        log::error!("Error to get response with request! -> {}", e);
        actix_web::error::ErrorInternalServerError(format!("Error to get response -> {}", e))
    })?;
    build_response(response).await
}

pub async fn handler_read_all_products(
    client: web::Data<Client>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/products/read_all_products", PRODUCT_SERVICE_URL);
    let response = send_request(&client, &url, None::<&Value>).await?;
    build_response(response).await
}

pub async fn handler_sort_product(
    client: web::Data<Client>,
    wanted_name: Option<web::Json<WantedName>>,
    wanted_price: Option<web::Json<WantedPrice>>,
    wanted_address: Option<web::Json<WantedAddress>>,
) -> Result<HttpResponse, actix_web::Error> {
    let url = format!("{}/products/sort_products", PRODUCT_SERVICE_URL);
    let response =
        get_response_from_sort(&client, &url, wanted_name, wanted_price, wanted_address).await?;
    build_response(response).await
}

async fn build_response(response: Response) -> Result<HttpResponse, actix_web::Error> {
    let status = response.status();
    let response_body = get_response_body(response).await?;
    let converted_status = convert_status_code_to_u16(&status)?;

    Ok(HttpResponse::build(converted_status).body(response_body))
}

async fn get_response_body(response: Response) -> Result<String, actix_web::Error> {
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

async fn get_response_from_sort(
    client: &web::Data<Client>,
    url: &str,
    wanted_name: Option<web::Json<WantedName>>,
    wanted_price: Option<web::Json<WantedPrice>>,
    wanted_address: Option<web::Json<WantedAddress>>,
) -> Result<Response, actix_web::Error> {
    match (wanted_name, wanted_price, wanted_address) {
        (Some(wanted_name), _, _) => send_request(client, url, Some(&wanted_name)).await,
        (_, Some(wanted_price), _) => send_request(client, url, Some(&wanted_price)).await,
        (_, _, Some(wanted_address)) => send_request(client, url, Some(&wanted_address)).await,
        _ => {
            log::error!("Invalid data from get_response_json!");
            Err(actix_web::error::ErrorBadRequest(
                "Invalid data to get_response_json!",
            ))
        }
    }
}

async fn send_request<T>(
    client: &Client,
    url: &str,
    body: Option<&T>,
) -> Result<Response, actix_web::Error>
where
    T: serde::Serialize,
{
    if let Some(body) = body {
        client.get(url).json(body).send().await.map_err(|e| {
            log::error!("Error to get response with json request! -> {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error to get response -> {}", e))
        })
    } else {
        client.get(url).send().await.map_err(|e| {
            log::error!("Error to get response with request! -> {}", e);
            actix_web::error::ErrorInternalServerError(format!("Error to get response -> {}", e))
        })
    }
}
