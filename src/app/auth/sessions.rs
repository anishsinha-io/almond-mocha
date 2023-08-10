use crate::app::{
    config::StorageLayer,
    datasources::{postgres, redis},
    dto::{CreateSession, DeleteSession},
};
use derive_more::Display;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use std::env;
use std::error::Error;

#[derive(Debug, Display, PartialEq, Eq)]
pub enum SessionInterface {
    #[display(fmt = "postgres")]
    Postgres,
    #[display(fmt = "redis")]
    Redis,
}

pub struct SessionManager {
    pub interface: SessionInterface,
}

impl SessionManager {
    pub fn new(interface: SessionInterface) -> Self {
        Self { interface }
    }

    pub fn create_signed_cookie(
        &self,
        session_id: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let private_key_raw = env::var("SESSION_SIGNING_KEY").unwrap();
        let data = serde_json::json!({ "session_id": session_id });

        let session_cookie_data = jsonwebtoken::encode(
            &Header::new(Algorithm::RS256),
            &data,
            &EncodingKey::from_rsa_pem(private_key_raw.as_bytes())?,
        )?;

        Ok(session_cookie_data)
    }

    pub async fn start_session(
        &self,
        storage_layer: &StorageLayer,
        data: CreateSession,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self.interface {
            SessionInterface::Postgres => {
                let pool = &storage_layer.pg;
                let id = postgres::auth::start_session(pool, data).await?;
                Ok(id)
            }
            SessionInterface::Redis => {
                let mut conn = storage_layer.redis.get().await?;
                let id = redis::auth::start_session(&mut conn, data).await?;
                Ok(id)
            }
        }
    }

    pub async fn end_session(
        &self,
        storage_layer: &StorageLayer,
        data: DeleteSession,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self.interface {
            SessionInterface::Postgres => {
                let pool = &storage_layer.pg;
                postgres::auth::end_session(pool, data).await?;
                Ok(())
            }
            SessionInterface::Redis => {
                let mut conn = storage_layer.redis.get().await?;
                redis::auth::end_session(&mut conn, data).await?;
                Ok(())
            }
        }
    }
}
