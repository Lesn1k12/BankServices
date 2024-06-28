#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::PgConnection;
    use std::env;
    use crate::models::{NewUser, LoginUser};
    use crate::db::DbPool;
    use dotenv;
    use std::sync::Once;
    use crate::handlers::{register, login};

    static INIT: Once = Once::new();

    pub fn create_pool() -> DbPool {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
    }

    fn initialize() {
        INIT.call_once(|| {
            dotenv::dotenv().ok();
            env_logger::init();
        });
    }

    #[actix_rt::test]
    async fn test_register() {
        initialize();

        let pool = create_pool();

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/auth/register", web::post().to(register))
        ).await;

        let new_user = NewUser {
            username: "user6".to_string(),
            password: "testpassword6".to_string(),
            role: "user".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&new_user)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;
        println!("Response status: {:?}", status);
        println!("Response body: {:?}", body);
        assert_eq!(status, actix_web::http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_login() {
        initialize();

        let pool = create_pool();

        // Спочатку зареєструємо нового користувача
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/auth/register", web::post().to(register))
                .route("/auth/login", web::post().to(login))
        ).await;

        let new_user = NewUser {
            username: "haha".to_string(),
            password: "haha4".to_string(),
            role: "user".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&new_user)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        // Тепер виконаємо логін для цього користувача
        let login_user = LoginUser {
            username: "haha".to_string(),
            password: "haha4".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&login_user)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        let status = resp.status();
        let body = test::read_body(resp).await;
        println!("Response status: {:?}", status);
        println!("Response body: {:?}", body);
        assert_eq!(status, actix_web::http::StatusCode::OK);

        // Перевірка, чи повернуто JWT токен
        let token: String = serde_json::from_slice(&body).expect("Failed to parse token from response");
        assert!(token.len() > 0);
    }
}
