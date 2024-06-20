// обробка запитів авторизації

use actix_web::{web, HttpResponse, Responder, http::StatusCode}; // Додавання http::StatusCode з actix-web
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(client: web::Data<Client>, login_req: web::Json<LoginRequest>) -> impl Responder {
    let auth_service_url = env::var("AUTH_URL").expect("AUTH_URL must be set");
    let url = format!("{}/login", auth_service_url);

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
