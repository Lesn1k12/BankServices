use crate::modules::{Product, WantedSortItem};
use actix_web::web;
use sqlx::PgPool;

pub async fn sort_products(
    pool: web::Data<PgPool>,
    wanted_sort_item: Option<web::Json<WantedSortItem>>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let all_products = read_all_products(pool).await?.into_inner();

    if let Some(wanted_sort_item) = wanted_sort_item {
        let all_products = sort_by_name(all_products, &wanted_sort_item);
        let all_products = sort_by_price(all_products, &wanted_sort_item);
        let all_products = sort_by_address(all_products, &wanted_sort_item);
        return Ok(web::Json(all_products));
    };

    Ok(web::Json(all_products))
}

fn sort_by_name(products: Vec<Product>, wanted_name: &web::Json<WantedSortItem>) -> Vec<Product> {
    if let Some(wanted_name) = &wanted_name.name {
        return products
            .into_iter()
            .filter(|product| product.name.contains(&wanted_name.to_lowercase()))
            .collect();
    }
    products
}

fn sort_by_price(products: Vec<Product>, wanted_price: &web::Json<WantedSortItem>) -> Vec<Product> {
    if wanted_price.highest_to_lowest.is_some() {
        sort_by_highest_price(products)
    } else if wanted_price.lowest_to_highest.is_some() {
        sort_by_lowest_price(products)
    } else {
        products
    }
}

fn sort_by_lowest_price(mut products: Vec<Product>) -> Vec<Product> {
    products.sort_by(|product, next_product| {
        product
            .price
            .partial_cmp(&next_product.price)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    products
}

fn sort_by_highest_price(mut products: Vec<Product>) -> Vec<Product> {
    products.sort_by(|product, next_product| {
        next_product
            .price
            .partial_cmp(&product.price)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    products
}

fn sort_by_address(
    products: Vec<Product>,
    wanted_address: &web::Json<WantedSortItem>,
) -> Vec<Product> {
    if let Some(wanted_country) = &wanted_address.country {
        if let Some(wanted_region) = &wanted_address.region {
            return products
                .into_iter()
                .filter(|product| {
                    product.storage_country.to_lowercase() == wanted_country.to_lowercase()
                        && product.storage_region.to_lowercase() == wanted_region.to_lowercase()
                })
                .collect();
        }
    };
    products
}

async fn read_all_products(
    pool: web::Data<PgPool>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let all_products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Error to read all products -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?;
    Ok(web::Json(all_products))
}
