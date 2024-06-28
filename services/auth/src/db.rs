use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::result::Result;
use log::info;


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn init_pool() -> Result<DbPool, String> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .map_err(|e| format!("Failed to create pool: {}", e))
}
