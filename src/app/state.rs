use super::{
    auth::{
        sessions::{SessionInterface, SessionManager},
        CredentialManager,
    },
    config::{Config, StorageLayer},
    dto::HashAlgorithm,
};
use std::env;

pub struct AppState {
    pub config: Config,
    pub storage_layer: StorageLayer,
    pub credential_manager: CredentialManager,
    pub session_manager: SessionManager,
}

impl AppState {
    pub async fn new(name: &str) -> Self {
        let config = Config::new(name).expect("error generating app configuration");
        let storage_layer = StorageLayer::new()
            .await
            .expect("error initializing storage backend");

        let hash_algorithm = match env::var("HASH_ALGORITHM")
            .unwrap_or("argon2".to_owned())
            .to_lowercase()
            .as_str()
        {
            "argon2" => HashAlgorithm::Argon2,
            "bcrypt" => HashAlgorithm::Bcrypt,
            _ => HashAlgorithm::Argon2,
        };

        let session_interface = match env::var("SESSION_INTERFACE")
            .unwrap_or("redis".to_owned())
            .to_lowercase()
            .as_str()
        {
            "postgresql" | "postgres" | "pg" => SessionInterface::Postgres,
            "redis" => SessionInterface::Redis,
            _ => SessionInterface::Redis,
        };

        let credential_manager = CredentialManager::new(hash_algorithm);
        let session_manager = SessionManager::new(session_interface);

        Self {
            config,
            storage_layer,
            credential_manager,
            session_manager,
        }
    }
}
