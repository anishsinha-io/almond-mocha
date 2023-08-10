use crate::app::{
    config::{Config, StorageLayer},
    datasources::{
        postgres,
        redis::{self, RedisConn},
    },
    dto::{CreateSession, DeleteSession},
};
use derive_more::Display;
use sqlx::{Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

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
    pub fn default() -> Self {
        Self {
            interface: SessionInterface::Postgres,
        }
    }

    pub fn new(interface: SessionInterface) -> Self {
        Self { interface }
    }

    pub fn create_signed_cookie(config: Config) {}

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
