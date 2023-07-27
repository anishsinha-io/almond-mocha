use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub struct AppState {
    pub name: String,
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new(name: &str) -> Self {
        let db_url = env::var("DATABASE_URL").expect("error loading database url");

        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(&db_url)
            .await
            .expect("error creating database connection pool");

        Self {
            name: name.to_owned(),
            pool,
        }
    }
}
