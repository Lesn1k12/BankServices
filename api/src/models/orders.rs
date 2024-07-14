use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Order {
    pub wanted_quantity: i32,
    pub product_id: i32,
    pub product_name: String,
    pub product_price: f64,
    pub product_category: String,
    pub product_storage_country: String,
    pub product_storage_region: String,
    pub product_storage_street: String,
    pub product_storage_quantity: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Orders {
    pub id: Option<i32>,
    pub user_id: i32,
    pub user_name: String,
    pub orders: Vec<Order>,
}
