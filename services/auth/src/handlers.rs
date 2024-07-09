use std::fmt::format;
use log::info;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::{hash, verify};
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::{User, NewUser, LoginUser, Claims};
use crate::schema::users::dsl::*;
use crate::db::DbPool;
use crate::schema::users::dsl::*;
use std::time::{SystemTime, UNIX_EPOCH};


const SECRET: &[u8] = b"nYSvA9hsWvSZT/AOMcmiNze/YGtkwEFUMfCbos0LTgM=";

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/index", web::get().to(index))
    );
}

pub async fn login(login_user: web::Json<LoginUser>, pool: web::Data<DbPool>) -> HttpResponse {

    let conn_result = pool.get();
    let mut conn = match conn_result {
        Ok(conn) => {
            info!("Successfully got DB connection from pool");
            conn
        },
        Err(e) => {
            info!("Failed to get DB connection from pool: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to get DB connection from pool: {:?}", e));
        }
    };

    let user_result = users.filter(username.eq(&login_user.username))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error loading user");

    let user = match user_result {
        Some(user) => {
            user
        },
        None => {
            info!("User not found");
            return HttpResponse::NotFound().body("User not found");
        }
    };

    if verify(&login_user.password, &user.password).unwrap_or(false) {
        info!("Password is correct");

        let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600;

        let claims = Claims {
            sub: user.username.clone(),
            exp: expiration as usize,
        };

        let token = claims.encode(SECRET);
        info!("Token: {:?}", &token);

        return HttpResponse::Ok().json(token);
    } else {
        info!("Password is incorrect");
        return HttpResponse::Unauthorized().body("Password is incorrect");
    }
}

pub async fn register(user: web::Json<NewUser>, pool: web::Data<DbPool>) -> HttpResponse {
    info!("Received register request: {:?}", user);

    info!("Username: {}", user.username);
    info!("Password: {}", user.password);
    info!("Role: {}", user.role);

    let conn_result = pool.get();
    let mut conn = match conn_result {
        Ok(conn) => {
            info!("Successfully got DB connection from pool");
            conn
        },
        Err(e) => {
            info!("Failed to get DB connection from pool: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to get DB connection from pool: {:?}", e));
        }
    };

    let existing_user = users.filter(username.eq(&user.username))
        .first::<User>(&mut conn)
        .optional()
        .expect("Error loading user");

    if let Some(_) = existing_user {
        info!("User already exists");
        return HttpResponse::Conflict().body("User already exists");
    }

    let hashed_password = match hash(&user.password, 4) {
        Ok(hashed) => {
            info!("Successfully hashed password");
            hashed
        },
        Err(e) => {
            info!("Failed to hash password: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Failed to hash password: {:?}", e));
        }
    };

    let new_user = NewUser {
        username: user.username.clone(),
        password: hashed_password,
        role: user.role.clone(),
    };
    info!("New user: {:?}", new_user);

    let insert_result = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn);

    match insert_result {
        Ok(_) => {
            info!("User registered successfully");
            HttpResponse::Ok().json("User registered")
        },
        Err(e) => {
            info!("Error saving new user: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error saving new user: {:?}", e))
        }
    }
}


pub async fn index() -> HttpResponse {
    info!("KURWA INDEX");
    HttpResponse::Ok().body("Index")
}

