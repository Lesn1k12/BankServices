use std::fmt::format;
use log::info;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::{hash, verify};
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::{User, NewUser, LoginUser, Claims};
use crate::schema::users::dsl::*;
use crate::db::DbPool;
use crate::schema::users::dsl::{users, username as db_username};
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

async fn pool_response(pool: web::Data<DbPool>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, HttpResponse> {
    match pool.get() {
        Ok(conn) => {
            info!("Successfully got DB connection from pool");
            Ok(conn)
        },
        Err(e) => {
            info!("Failed to get DB connection from pool: {:?}", e);
            Err(HttpResponse::InternalServerError().body(format!("Failed to get DB connection from pool: {:?}", e)))
        }
    }
}

async fn find_user(conn: &mut PgConnection, input_username: &str) -> Result<Option<User>, HttpResponse> {
    users
        .filter(db_username.eq(&input_username))
        .first::<User>(conn)
        .optional()
        .map_err(|_| {
            info!("Error loading user");
            HttpResponse::InternalServerError().body("Error loading user")
        })
}

async fn verify_password_and_generate_token(user: User, input_password: &str) -> Result<String, HttpResponse> {
    if verify(input_password, &user.password).unwrap_or(false) {
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

        Ok(token)
    } else {
        info!("Password is incorrect");
        Err(HttpResponse::Unauthorized().body("Password is incorrect"))
    }
}

async fn create_new_user(user: NewUser, conn: &mut PgConnection) -> Result<(), HttpResponse> {
    let hashed_password = match hash(&user.password, 4) {
        Ok(hashed) => {
            info!("Successfully hashed password");
            hashed
        },
        Err(e) => {
            info!("Failed to hash password: {:?}", e);
            return Err(HttpResponse::InternalServerError().body(format!("Failed to hash password: {:?}", e)));
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
        .execute(conn);

    match insert_result {
        Ok(_) => {
            info!("User registered successfully");
            Ok(())
        },
        Err(e) => {
            info!("Error saving new user: {:?}", e);
            Err(HttpResponse::InternalServerError().body(format!("Error saving new user: {:?}", e)))
        }
    }
}

pub async fn login(login_user: web::Json<LoginUser>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = match pool_response(pool).await {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let user = match find_user(&mut conn, &login_user.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            info!("User not found");
            return HttpResponse::NotFound().body("User not found");
        },
        Err(resp) => return resp,
    };

    match verify_password_and_generate_token(user, &login_user.password).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(resp) => resp,
    }
}

pub async fn register(user: web::Json<NewUser>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = match pool_response(pool).await {
        Ok(conn) => conn,
        Err(resp) => return resp,
    };

    let user = match find_user(&mut conn, &user.username).await {
        Ok(Some(_)) => {
            info!("User already exists");
            return HttpResponse::Conflict().body("User already exists");
        },
        Ok(None) => user.into_inner(),
        Err(resp) => return resp,
    };

    match create_new_user(user, &mut conn).await {
        Ok(_) => HttpResponse::Ok().json("User registered"),
        Err(resp) => resp,
    }
}

pub async fn index() -> HttpResponse {
    info!(" INDEX");
    HttpResponse::Ok().body("Index")
}

