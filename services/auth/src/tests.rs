#[actix_rt::test]
async fn test_authorization_success() {
    use crate::authorization;
    use crate::structures::{AuthRequest, AuthResponse, User};
    use actix_web::dev::ServiceResponse;
    use actix_web::test::{read_body, read_body_json, TestRequest};
    use actix_web::{test, web, App, HttpResponse};
    use sqlx::{query, SqlitePool};
    use std::sync::Arc;

    let pool = setup_test_db().await;

    // Insert a test user into the database
    let test_user = User {
        id: 1,
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
    };
    query("INSERT INTO users (id, username, password) VALUES (?, ?, ?)")
        .bind(test_user.id)
        .bind(&test_user.username)
        .bind(&test_user.password)
        .execute(&pool)
        .await
        .unwrap();

    let auth_request = AuthRequest {
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/auth", web::post().to(authorization)),
    )
        .await;

    let req = TestRequest::post()
        .uri("/auth")
        .set_json(&auth_request)
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    let response_body: AuthResponse = read_body_json(resp).await;
    assert_eq!(response_body.token, "example_token");
    assert_eq!(response_body.expires_in, 3600);
}

#[actix_rt::test]
async fn test_authorization_failure() {
    use crate::authorization;
    use crate::structures::{AuthRequest, AuthResponse, User};
    use actix_web::dev::ServiceResponse;
    use actix_web::test::{read_body, read_body_json, TestRequest};
    use actix_web::{test, web, App, HttpResponse};
    use sqlx::{query, SqlitePool};
    use std::sync::Arc;

    let pool = setup_test_db().await;

    // Insert a test user into the database
    let test_user = User {
        id: 1,
        username: "testuser".to_string(),
        password: "testpassword".to_string(),
    };
    query("INSERT INTO users (id, username, password) VALUES (?, ?, ?)")
        .bind(test_user.id)
        .bind(&test_user.username)
        .bind(&test_user.password)
        .execute(&pool)
        .await
        .unwrap();

    let auth_request = AuthRequest {
        username: "testuser".to_string(),
        password: "wrongpassword".to_string(),
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/auth", web::post().to(authorization)),
    )
        .await;

    let req = TestRequest::post()
        .uri("/auth")
        .set_json(&auth_request)
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);

    let response_body = read_body(resp).await;
    assert_eq!(response_body, "Incorrect Password or Login");
}

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();

    query(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        );
        "#,
    )
        .execute(&pool)
        .await
        .unwrap();

    pool
}
