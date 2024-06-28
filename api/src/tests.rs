use actix_web::{test, web, App};
use actix_web::http::StatusCode;
use dotenv::dotenv;
use reqwest::Client;
use std::env;


// Імпорт ваших модулів
use crate::handlers;
use crate::routes;


#[cfg(test)]
mod tests_auth {
    use super::*;
    use actix_rt::test;

    #[actix_rt::test]
    async fn test_index() {
        dotenv().ok();
        env::set_var("AUTH_URL", "http://localhost:8081"); // або ваш фактичний URL для тестування

        let client = Client::new();

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(client))
                .configure(routes::init)
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/index")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Привіт, світ!");
    }
}