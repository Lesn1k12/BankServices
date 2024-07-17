// обробка запитів авторизації

use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
    role: String,
}

const AUTH_SERVICE_URL: &str = "http://localhost:8081";

pub async fn login(
    client: web::Data<Client>,
    login_req: web::Json<LoginRequest>,
) -> impl Responder {
    let url = format!("{}/auth/login", AUTH_SERVICE_URL);

    let res = client.post(&url).json(&*login_req).send().await;

    handle_response(res).await
}

pub async fn register(
    client: web::Data<Client>,
    register_req: web::Json<LoginRequest>,
) -> impl Responder {
    info!("Registering user");
    info!("Request: {:?}", &register_req);
    let url = format!("{}/auth/register", AUTH_SERVICE_URL);

    let res = client.post(&url).json(&*register_req).send().await;
    info!("jest json");
    info!("Response: {:?}", res);

    handle_response(res).await
}

pub async fn index(client: web::Data<Client>) -> impl Responder {
    info!("Auth service index");
    let url = format!("{}/auth/index", AUTH_SERVICE_URL);

    info!("Sending request to auth service at: {}", url);

    let res = client.get(&url).send().await;

    handle_response(res).await
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
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


