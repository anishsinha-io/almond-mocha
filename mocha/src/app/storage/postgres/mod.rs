use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod auth;
pub mod posts;
pub mod spaces;
pub mod stickers;
pub mod users;

pub async fn create_pool(max_connections: u32) -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("error loading database url");

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&db_url)
        .await
    // .expect("error creating database connection pool")
}
