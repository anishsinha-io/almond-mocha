use sqlx::{Pool, Postgres};
use std::env;

use super::{auth::CredentialManager, datasources::postgres, launch::LaunchMode};

pub struct AppState {
    pub name: String,
    pub pool: Pool<Postgres>,
    pub manager: CredentialManager,
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

        let manager = CredentialManager::default();

        Self {
            name: name.to_owned(),
            pool,
            manager,
            launch_mode,
        }
    }
}
