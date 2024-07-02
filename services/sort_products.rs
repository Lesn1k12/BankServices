pub async fn sort_products(
    pool: web::Data<PgPool>,
    wanted_name: Option<web::Json<WantedName>>,
    wanted_price: Option<web::Json<WantedPrice>>,
    wanted_address: Option<web::Json<WantedAddress>>,
) -> Result<web::Json<Vec<Product>>, actix_web::Error> {
    let mut all_products = read_all_products(pool).await?.into_inner().clone();

    if let Some(wanted_name) = wanted_name {
        all_products = sort_product_by_name(all_products, wanted_name)?;
    }

    if let Some(wanted_price) = wanted_price {
        if wanted_price.lowest_to_highest.is_some() {
            all_products = sort_product_from_lowest_to_highest_price(all_products)?
        } else {
            all_products = sort_product_for_highest_to_lowest_price(all_products)?
        }
    }

    if let Some(wanted_address) = wanted_address {
        all_products = sort_by_region(all_products, wanted_address)?
    }

    Ok(web::Json(all_products))
}

fn sort_product_by_name(
    all_products: Vec<Product>,
    wanted_name: web::Json<WantedName>,
) -> Result<Vec<Product>, actix_web::Error> {
    let sorted_product_by_name = all_products
        .into_iter()
        .filter(|product| product.name.contains(&wanted_name.name.to_lowercase()))
        .collect();

    Ok(sorted_product_by_name)
}

fn sort_product_from_lowest_to_highest_price(
    all_products: Vec<Product>,
) -> Result<Vec<Product>, actix_web::Error> {
    let mut sorted_by_price_products = all_products;
    sorted_by_price_products.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    Ok(sorted_by_price_products)
}

fn sort_product_for_highest_to_lowest_price(
    all_products: Vec<Product>,
) -> Result<Vec<Product>, actix_web::Error> {
    let mut sorted_by_price_products = all_products;
    sorted_by_price_products.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());

    Ok(sorted_by_price_products)
}

fn sort_by_region(
    all_products: Vec<Product>,
    wanted_address: web::Json<WantedAddress>,
) -> Result<Vec<Product>, actix_web::Error> {
    let sorted_by_region_products = all_products
        .into_iter()
        .filter(|product| {
            product.storage_country.to_lowercase() == wanted_address.country.to_lowercase()
                && product.storage_region.to_lowercase() == wanted_address.region.to_lowercase()
        })
        .collect();

    Ok(sorted_by_region_products)
}
