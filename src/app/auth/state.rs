use crate::app::dto::HashAlgorithm;

use super::{
    sessions::{SessionInterface, SessionManager},
    CredentialManager,
};
use std::env;

pub struct AuthState {
    pub credential_manager: CredentialManager,
    pub session_manager: SessionManager,
}

impl AuthState {
    pub fn new() -> Self {
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
            credential_manager,
            session_manager,
        }
    }
}
