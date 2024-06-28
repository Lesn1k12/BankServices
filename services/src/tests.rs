use crate::create_product::create_product;
use crate::modules::Product;
use crate::modules::ProductUpdate;
use crate::read_product::read_product;
use crate::remove_product::remove_product;
use crate::update_product::update_product;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::initial_logger;
    use actix_web::{test, web, App};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Executor, PgPool};
    use std::env;
    use std::sync::Arc;
    #[actix_rt::test]
    async fn test_create_product() {
        let pool = setup_mocked_pool().await;
        let pool_data = web::Data::new(pool);
        let product = web::Json(Product {
            id: None,
            name: String::from("NEW"),
            price: 150.00,
            category: String::from("NEW"),
            storage_country: String::from("NEW"),
            storage_region: String::from("NEW"),
            storage_street: String::from("ZV"),
            storage_quantity: 5,
        });
        let mut app = test::init_service(
            App::new()
                .app_data(pool_data.clone()) // Use app_data instead of set
                .service(web::resource("/create_product").route(web::post().to(create_product))),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/create_product")
            .set_json(&product)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    async fn setup_mocked_pool() -> PgPool {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:7AIQF41SDJZ@localhost:5432/postgres")
            .await
            .unwrap();
        pool
    }

    #[actix_rt::test]
    async fn test_update_product() {
        let pool = setup_mocked_pool().await;
        let data_pool = web::Data::new(pool);

        let product_id = web::Path::from(57i32); // предполагаем, что продукт с таким ID существует
        let new_product_data = ProductUpdate {
            name: Some(String::from("NEWNEW")),
            price: Some(100.0),
            category: None,
            storage_country: Some(String::from("NEWNEWNEW")),
            storage_region: Some(String::from("NEWNEWNEW")),
            storage_street: None,
            storage_quantity: Some(10),
        };
        let new_product_json = web::Json(new_product_data);

        let result = update_product(data_pool.clone(), product_id, new_product_json).await;
        assert!(result.is_ok());
    }
    #[actix_rt::test]
    async fn test_remove_product() {
        let pool = setup_mocked_pool().await;

        // Настройка приложения
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/product/{id}", web::delete().to(remove_product)),
        )
        .await;

        // Отправка запроса на удаление продукта
        let req = test::TestRequest::delete().uri("/product/46").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Проверка статуса ответа
        assert!(resp.status().is_success());

        // Проверка, что продукт действительно удален
        let product_exists = sqlx::query("SELECT * FROM products WHERE id = $1")
            .bind(1)
            .fetch_optional(&pool)
            .await
            .unwrap()
            .is_some();

        assert!(!product_exists);
    }

    #[actix_rt::test]
    async fn test_read_product() {
        initial_logger();
        let pool = setup_mocked_pool().await;

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/product/{id}", web::get().to(read_product)),
        )
        .await;

        let req = test::TestRequest::get().uri("/product/52").to_request();
        let resp: Product = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.name, "AYE");
        assert_eq!(resp.price, 150 as f64);
    }

    #[actix_rt::test]
    async fn test_read_all_product() {
        let pool = setup_mocked_pool().await;
        let pool_data = web::Data::new(pool);

        let app = test::init_service(
            App::new()
                .app_data(pool_data.clone())
                .route("/products", web::get().to(read_all_product)),
        )
        .await;

        let req = test::TestRequest::get().uri("/products").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        // Дополнительно можно проверить и содержимое ответа
        let result: Vec<Product> = test::read_body_json(resp).await;
        assert!(result.len() > 2, "Should return at least one product");
    }
}
