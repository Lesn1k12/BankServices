use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const TASKS_SERVICE_URL: &str = "http://localhost:8084";

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatedTask {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

pub async fn create_task(client: web::Data<Client>, task: web::Json<NewTask>) -> HttpResponse {
    let url = format!("{}/orders/create_order", TASKS_SERVICE_URL);
    let res = client.post(&url).json(&*task).send().await;

    handle_response(res).await
}

pub async fn get_tasks(client: web::Data<Client>) -> HttpResponse {
    let url = format!("{}/orders/create_order", TASKS_SERVICE_URL);
    let res = client.get(&url).send().await;

    handle_response(res).await
}

pub async fn get_task_by_id(client: web::Data<Client>, task_id: web::Path<i32>, ) -> HttpResponse {
    let url = format!("{}/orders/create_order/{}", TASKS_SERVICE_URL, task_id);
    let res = client.get(&url).send().await;

    handle_response(res).await
}

pub async fn update_task(client: web::Data<Client>, task_id: web::Path<i32>, task: web::Json<UpdatedTask>, ) -> HttpResponse {
    let url = format!("{}/orders/create_order/{}", TASKS_SERVICE_URL, task_id);
    let res = client.put(&url).json(&*task).send().await;

    handle_response(res).await
}

pub async fn delete_task(client: web::Data<Client>, task_id: web::Path<i32>, ) -> HttpResponse {
    let url = format!("{}/orders/create_order", TASKS_SERVICE_URL);
    let res = client.delete(&url).json(&*task_id).send().await;

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
