use crate::modules::{Product, WantedSortItem};
use actix_web::web;
use sqlx::PgPool;

pub async fn sort_products(
    pool: web::Data<PgPool>,
    wanted_sort_item: Option<web::Json<WantedSortItem>>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let all_products = read_all_products(pool).await?.into_inner();

    if let Some(wanted_sort_item) = wanted_sort_item {
        return Ok(web::Json(merge_sorts(all_products, &wanted_sort_item)?));
    };

    Ok(web::Json(all_products))
}

fn merge_sorts(mut products: Vec<Product>, wanted_item: &web::Json<WantedSortItem>) -> Result<Vec<Product>, actix_web::Error> {
    filter_and_sort(&mut products, &wanted_item.name, |product| product.name.to_lowercase());
    filter_and_sort(&mut products, &wanted_item.country, |product| product.storage_country.to_lowercase());
    filter_and_sort(&mut products, &wanted_item.region, |product| product.storage_region.to_lowercase());
    filter_and_sort(&mut products, &wanted_item.category, |product| product.category.to_lowercase());
    sort_by_price(&mut products, &wanted_item)?;

    Ok(products)
}

fn filter_and_sort<T, F>(products: &mut Vec<Product>, wanted_item: &Option<T>, field_extractor: F)
    where
        F: Fn(&Product) -> String,
        T: std::fmt::Display,
{
    if let Some(wanted_value) = wanted_item {
        products.retain(|product| {
            field_extractor(product)
                .to_lowercase()
                .contains(&wanted_value.to_string().to_lowercase())
        });

        products.sort_by_key(field_extractor);
    }
}


fn sort_by_price(products: &mut Vec<Product>, wanted_item: &web::Json<WantedSortItem>) -> Result<(), actix_web::Error>{
    match (wanted_item.lowest_to_highest, wanted_item.highest_to_lowest){
        (Some(_), Some(_)) => {
            log::error!("You can sort by price only with 1 parameter!");
            return Err(actix_web::error::ErrorInternalServerError("You can sort by price only with 1 parameter!"));
        }

        (Some(true), _) => {
            products.sort_by(|product, next_product| {
                product
                    .price
                    .partial_cmp(&next_product.price)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        (_, Some(true)) => {
            products.sort_by(|product, next_product| {
                next_product
                    .price
                    .partial_cmp(&product.price)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        (_, _) => ()
    }

    Ok(())
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
