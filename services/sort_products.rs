use crate::modules::{Product, WantedAddress, WantedName, WantedPrice};
use crate::read_all_products::read_all_products;
use actix_web::web;
use sqlx::PgPool;

pub async fn sort_products(
    pool: web::Data<PgPool>,
    wanted_name: Option<web::Json<WantedName>>,
    wanted_price: Option<web::Json<WantedPrice>>,
    wanted_address: Option<web::Json<WantedAddress>>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let all_products = read_all_products(pool).await?.into_inner();

    let sorted_products = match (wanted_name, wanted_price, wanted_address) {
        (Some(name), _, _) => sort_by_name(all_products, name),
        (_, Some(price), _) => sort_by_price(all_products, price),
        (_, _, Some(address)) => sort_by_address(all_products, address),
        _ => {
            log::error!("Not correct arguments to sort_product!");
            all_products
        }
    };

    Ok(web::Json(sorted_products))
}

fn sort_by_name(mut products: Vec<Product>, wanted_name: web::Json<WantedName>) -> Vec<Product> {
    products
        .into_iter()
        .filter(|product| product.name.contains(&wanted_name.name.to_lowercase()))
        .collect()
}

fn sort_by_price(products: Vec<Product>, wanted_price: web::Json<WantedPrice>) -> Vec<Product> {
    if wanted_price.highest_to_lowest.is_some() {
        sort_by_highest_price(products)
    } else {
        sort_by_lowest_price(products)
    }
}

fn sort_by_lowest_price(mut products: Vec<Product>) -> Vec<Product> {
    products.sort_by(|a, b| {
        a.price
            .partial_cmp(&b.price)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    products
}

fn sort_by_highest_price(mut products: Vec<Product>) -> Vec<Product> {
    products.sort_by(|a, b| {
        b.price
            .partial_cmp(&a.price)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    products
}

fn sort_by_address(
    mut products: Vec<Product>,
    wanted_address: web::Json<WantedAddress>,
) -> Vec<Product> {
    products
        .into_iter()
        .filter(|product| {
            product.storage_country.to_lowercase() == wanted_address.country.to_lowercase()
                && product.storage_region.to_lowercase() == wanted_address.region.to_lowercase()
        })
        .collect()
}

