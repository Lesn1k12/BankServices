use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use anyhow::Result;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Result<r2d2::Pool<ConnectionManager<PgConnection>>, diesel::r2d2::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    diesel::r2d2::Pool::builder().build(manager)
}

