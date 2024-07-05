use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct WantedSortItem {
    pub name: Option<String>,
    pub highest_to_lowest: Option<bool>,
    pub lowest_to_highest: Option<bool>,
    pub category: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
}
