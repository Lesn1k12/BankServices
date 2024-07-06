use crate::modules::{Product, WantedSortItem};
use actix_web::web;
use sqlx::PgPool;

pub async fn sort_products(
    pool: web::Data<PgPool>,
    wanted_sort_item: Option<web::Json<WantedSortItem>>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let all_products = read_all_products(pool).await?.into_inner();

    if let Some(wanted_sort_item) = wanted_sort_item {
        return Ok(web::Json(merge_sorts(all_products, wanted_sort_item)?));
    };

    Ok(web::Json(all_products))
}

fn merge_sorts(mut products: Vec<Product>, wanted_item: web::Json<WantedSortItem>) -> Result<Vec<Product>, actix_web::Error> {
    products = sort_by_name(products, &wanted_item);
    sort_by_price(&mut products, &wanted_item)?;
    products = sort_by_terrain(products, &wanted_item);
    products = sort_by_category(products, &wanted_item);

    Ok(products)
}

fn sort_by_name(mut products: Vec<Product>, wanted_item: &web::Json<WantedSortItem>) -> Vec<Product>{
    if let Some(wanted_name) = &wanted_item.name {
        products = products
            .into_iter()
            .filter(|product| {
                product
                    .name
                    .to_lowercase()
                    .contains(&wanted_name.to_lowercase())
            })
            .collect();

        products.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    };

    products
}

fn sort_by_price(products: &mut Vec<Product>, wanted_item: &web::Json<WantedSortItem>) -> Result<(), actix_web::Error>{
    match (wanted_item.lowest_to_highest, wanted_item.highest_to_lowest){
        (Some(_), Some(_)) => {
            log::error!("You can sort by price only with 1 parameter!");
            return Err(actix_web::error::ErrorInternalServerError("You can sort by price only with 1 parameter!"));
        }

        (Some(wanted_price), _) => {
            if wanted_price {
                products.sort_by(|product, next_product| {
                    product
                        .price
                        .partial_cmp(&next_product.price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }

        (_, Some(wanted_price)) => {
            if wanted_price {
                products.sort_by(|product, next_product| {
                    next_product
                        .price
                        .partial_cmp(&product.price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }
        
        (_, _) => ()
    }

    Ok(())
}

fn sort_by_terrain(mut products: Vec<Product>, wanted_item: &web::Json<WantedSortItem>) -> Vec<Product>{
    if let (Some(wanted_country), Some(wanted_region)) = (&wanted_item.country, &wanted_item.region) {
        products = products
            .into_iter()
            .filter(|product| {
                product
                    .storage_country
                    .eq_ignore_ascii_case(&wanted_country)
                    && product.storage_region.eq_ignore_ascii_case(wanted_region)
            })
            .collect();
    };

    products
}

fn sort_by_category(mut products: Vec<Product>, wanted_item: &web::Json<WantedSortItem>) -> Vec<Product>{
    if let Some(wanted_category) = &wanted_item.category {
        products = products
            .into_iter()
            .filter(|product| product.category.eq_ignore_ascii_case(&wanted_category))
            .collect();
    }

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
