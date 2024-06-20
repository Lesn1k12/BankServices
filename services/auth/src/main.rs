use crate::models::{AuthRequest, AuthResponse, User};
use actix_web::{web, HttpResponse};
use sqlx::query;
use sqlx::SqlitePool;
use anyhow::Result;


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect("postgres://username:password@localhost/auth_db")
        .await
        .inspect_err(|e| log::error!("Error connection to DB -> {}", e))?;

    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let _ = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/auth", post().to(authorization))
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await;

    Ok(())
}



