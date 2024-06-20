use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use rusqlite::{params, Connection};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::{LoginRequest, User};

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use rusqlite::{params, Connection};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::{LoginRequest, User};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login))
    )
    .service(
        web::resource("/register")
            .route(web::post().to(register))
    )
    .service(
        web::resource("/user/{id}")
            .route(web::get().to(get_user))
    );
}

async fn login(login_req: web::Json<LoginRequest>, pool:web::Data<>) -> impl Responder {
    let conn = pool.aquire().await;

    let user: Option<User> = sqlx::query_as!(
        "SELECT id, username, password FROM users WHERE username = ?1",
    )
        .bind(&login_req.username)
        .fetch_optional(&pool)
        .await;
    if let Some(user) = user {
        if user.password == login_req.password {
            let token = encode(
                &Header::default(),
                &user,
                &EncodingKey::from_secret("secret".as_ref()));
            return HttpResponse::Ok().json(AuthResponse { token, expires_in: 3600 });
        }
    }
    HttpResponse::Unauthorized().finish()
}

async fn register() -> impl Responder {
    let conn = pool.aquire().await;

    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
    )
        .bind(&login_req.username)
        .bind(&login_req.password)
        .execute(conn)
        .await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// pub async fn authorization(
//     auth_request: web::Json<AuthRequest>,
//     pool: web::Data<SqlitePool>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let user = get_user(&auth_request, pool).await?;
//     let http_response = password_verification(&user, &auth_request).await?;
//     Ok(http_response)
// }

async fn get_user(
    auth_request: &web::Json<AuthRequest>,
    pool: web::Data<SqlitePool>,
) -> Result<Option<User>, actix_web::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(&auth_request.username)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| {
            log::error!("Database query error: {}", e);
            actix_web::error::ErrorInternalServerError("Internal Server Error")
        })?;
    Ok(user)
}

// async fn password_verification(
//     user: &Option<User>,
//     auth_request: &web::Json<AuthRequest>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     if let Some(user) = user {
//         if user.password == auth_request.password {
//             let response = AuthResponse {
//                 token: "example_token".to_string(),
//                 expires_in: 3600,
//             };
//             return Ok(HttpResponse::Ok().json(response));
//         }
//     }
//
//     return Ok(HttpResponse::Unauthorized().body("Incorrect Password or Login"));
// }
