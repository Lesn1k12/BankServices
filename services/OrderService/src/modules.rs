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
    pub orders: Vec<Order>
}

impl Orders{
    pub fn new() -> Self{
        Orders{
            id: None,
            user_id: 0,
            user_name: String::new(),
            orders: Vec::new(),
        }
    }

    pub fn update(&mut self, id: i32, user_id: i32, user_name: String, orders: Vec<Order>){
        self.id = Some(id);
        self.user_id = user_id;
        self.orders = orders;
        self.user_name = user_name;
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct ProductUpdate {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub category: Option<String>,
    pub storage_country: Option<String>,
    pub storage_region: Option<String>,
    pub storage_street: Option<String>,
    pub storage_quantity: Option<i32>,
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

