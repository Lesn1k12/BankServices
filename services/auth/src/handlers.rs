use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::{User, NewUser, LoginUser, Claims};
use crate::schema::users::dsl::*;
use crate::db::DbPool;

pub async fn login(
    login_req: web::Json<LoginUser>, 
    pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let user_result = users
        .filter(username.eq(&login_req.username))
        .first::<User>(&mut conn);

    match user_result {
        Ok(user) => {
            if verify(&login_req.password, &user.password).unwrap() {
                let claims = Claims {
                    sub: user.username.clone(),
                    exp: 10000000000,
                };

                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"key")).unwrap();
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().json("Invalid password")
            }
        },
        Err(_) => {
            HttpResponse::Unauthorized().json("User not found")
        }
    }
}

pub async fn register(
    user: web::Json<NewUser>, 
    pool: web::Data<DbPool>
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get db connection from pool");

    let hashed_password = hash(&user.password, 4).unwrap();
    let new_user = NewUser {
        username: user.username.clone(),
        password: hashed_password,
        role: "user".to_string(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error saving new user");

    HttpResponse::Ok().json("User registered")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth")
            .route(web::post().to(login))
    )
    .service(
        web::resource("/register")
            .route(web::post().to(register))
    );
}
