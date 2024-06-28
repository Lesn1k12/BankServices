// обробка запитів авторизації

use actix_web::{get, web, HttpResponse, Responder, http::StatusCode}; // Додавання http::StatusCode з actix-web
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;
use log::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
    role: String,
}

pub async fn login(client: web::Data<Client>, login_req: web::Json<LoginRequest>) -> impl Responder {
    let auth_service_url = "http://localhost:8081";
    let url = format!("{}/auth/login", auth_service_url);

    let res = client.post(&url)
        .json(&*login_req)
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

pub async fn register(client: web::Data<Client>, register_req: web::Json<LoginRequest>) -> impl Responder {
    info!("Registering user");
    info!("Request: {:?}", &register_req);
    let auth_service_url = "http://localhost:8081";
    let url = format!("{}/auth/register", auth_service_url);

    let res = client.post(&url)
        .json(&*register_req)
        .send()
        .await;
    info!("jest json");
    info!("Response: {:?}", res);


    match res {
        Ok(response) => {
            info!("dobrze");
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        },
        Err(_) => {
            info!("kurwa nie działa");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn index(client: web::Data<Client>) -> impl Responder {
    info!("Auth service index");
    let auth_service_url = "http://localhost:8081";
    let url = format!("{}/auth/index", auth_service_url);

    info!("Sending request to auth service at: {}", url);

    let res = client.get(&url).send().await;

    match res {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            info!("Received response with status: {}", status);
            HttpResponse::build(StatusCode::from_u16(status.as_u16()).unwrap()).body(body)
        },
        Err(e) => {
            info!("Error connecting to auth service: {}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}