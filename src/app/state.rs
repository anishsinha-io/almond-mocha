use sqlx::{Pool, Postgres};
use std::env;

use super::{
    auth::{sessions::SessionManager, CredentialManager},
    datasources::{
        postgres,
        redis::{self, RedisPool},
    },
    launch::LaunchMode,
};

pub struct AppState {
    pub name: String,
    pub pool: Pool<Postgres>,
    pub cache: RedisPool,
    pub credential_manager: CredentialManager,
    pub session_manager: SessionManager,
    pub launch_mode: LaunchMode,
}

impl AppState {
    pub async fn new(name: &str) -> Self {
        let launch_mode = match env::var("LAUNCH_MODE")
            .expect("LAUNCH_MODE environment variable is not set")
            .as_str()
        {
            "development" => LaunchMode::Development,
            "testing" => LaunchMode::Testing,
            "staging" => LaunchMode::Staging,
            _ => LaunchMode::Production,
        };

        let pool = postgres::create_pool(100)
            .await
            .expect("error creating postgresql connection pool");

        let credential_manager = CredentialManager::default();
        let session_manager = SessionManager::default();

        let cache = redis::create_pool()
            .await
            .expect("error creating redis pool");

        Self {
            name: name.to_owned(),
            pool,
            cache,
            credential_manager,
            session_manager,
            launch_mode,
        }
    }
}
