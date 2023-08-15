use crate::app::{
    config::StorageLayer,
    dto::auth::{CreateSession, DeleteSession, GetSessionById},
    entities::auth::Session,
    storage::{postgres, redis},
};
use derive_more::Display;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde_json::Value;
use std::{env, time::SystemTime};
use std::{error::Error, time::UNIX_EPOCH};

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

    pub fn verify_session_signature(
        &self,
        cookie: &str,
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let public_key = env::var("SESSION_VERIFYING_KEY")?;
        let decoded = jsonwebtoken::decode::<Value>(
            cookie,
            &DecodingKey::from_rsa_pem(public_key.as_bytes())?,
            &Validation::new(Algorithm::RS256),
        )
        .map_err(|e| {
            log::error!("{}", e);
            e
        })?;

        Ok(decoded.claims)
    }

    pub async fn check_session(
        &self,
        storage_layer: &StorageLayer,
        cookie: &str,
    ) -> Result<Session, Box<dyn Error + Send + Sync>> {
        let cookie_data = self.verify_session_signature(cookie)?;

        let session_id = cookie_data["session_id"].as_str().unwrap_or("");
        let dto = GetSessionById {
            id: session_id.to_owned(),
        };

        match self.interface {
            SessionInterface::Postgres => {
                Ok(postgres::auth::get_session(&storage_layer.pg, dto).await?)
            }
            SessionInterface::Redis => {
                let mut conn = storage_layer.redis.get().await?;
                Ok(redis::auth::get_session(&mut conn, dto).await?)
            }
        }
    }

    pub fn create_signed_cookie(
        &self,
        session_id: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let private_key = env::var("SESSION_SIGNING_KEY").unwrap();

        let mut exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        exp += 60 * 60 * 24 * 52 * 3;
        let data = serde_json::json!({ "session_id": session_id, "exp": exp });

        let session_cookie_data = jsonwebtoken::encode::<Value>(
            &Header::new(Algorithm::RS256),
            &data,
            &EncodingKey::from_rsa_pem(private_key.as_bytes())?,
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
