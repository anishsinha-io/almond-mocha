use super::{
    datasources::{
        postgres,
        redis::{self, RedisPool},
    },
    launch::LaunchMode,
};
use std::env;

use derive_more::{Display, Error};
use sqlx::{Pool, Postgres};

pub struct Config {
    pub name: String,
    pub symmetric_secret: Vec<u8>,
    pub launch_mode: LaunchMode,
}

pub struct StorageLayer {
    pub pg: Pool<Postgres>,
    pub redis: RedisPool,
}

#[derive(Display, Debug, Error)]
pub enum InitError {
    #[display(fmt = "missing app secret")]
    Secret,
    #[display(fmt = "error initializing caching layer")]
    InitRedis,
    #[display(fmt = "error initializing sql connection pool")]
    InitPostgres,
}

impl StorageLayer {
    pub async fn new() -> Result<Self, InitError> {
        let sql = postgres::create_pool(100)
            .await
            .map_err(|_| InitError::InitPostgres)?;

        let cache = redis::create_pool()
            .await
            .map_err(|_| InitError::InitRedis)?;

        Ok(Self {
            pg: sql,
            redis: cache,
        })
    }
}

impl Config {
    pub fn new(name: &str) -> Result<Self, InitError> {
        let symmetric_secret = env::var("APP_SECRET")
            .map(|s| s.as_bytes().to_vec())
            .map_err(|_| InitError::Secret)?;

        let launch_mode = match env::var("LAUNCH_MODE")
            .expect("LAUNCH_MODE environment variable is not set")
            .as_str()
        {
            "development" => LaunchMode::Development,
            "testing" => LaunchMode::Testing,
            "staging" => LaunchMode::Staging,
            _ => LaunchMode::Production,
        };

        Ok(Config {
            name: name.to_owned(),
            symmetric_secret,
            launch_mode,
            // session_interface,
        })
    }
}
