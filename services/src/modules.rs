use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_in: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub to: String,      // Куда идет отправка сообщений.
    pub subject: String, // Заголовок(та черная хуйня вначале письма)
    pub body: String,    // Тело запроса. Основаная инфа!
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
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
