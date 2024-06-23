#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use actix_rt::test as actix_test;

    #[actix_test]
    async fn test_register_user() {
        let pool = init_pool().expect("Failed to create pool.");
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .configure(crate::handlers::init)
        ).await;

        let user_data = NewUser {
            username: "testuser".to_string(),
            password: "password123".to_string(),
            role: "user".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&user_data)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success(), "Registration should be successful");
    }

    #[actix_test]
    async fn test_login_user() {
        let pool = init_pool().expect("Failed to create pool.");
        let mut app = test::init_service(
            App::new()
                .data(pool.clone())
                .configure(crate::handlers::init)
        ).await;

        let login_data = LoginUser {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth")
            .set_json(&login_data)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success(), "Login should be successful");
    }
}
