use crate::app::dto::{CreateSession, DeleteSession};
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

    pub fn create_signed_cookie() {}

    async fn start_session_postgres(
        &self,
        pool: &Pool<Postgres>,
        data: CreateSession,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let query = "insert into jen.sessions (user_id, data) values ($1, $2) returning id";

        let session: (Uuid,) = sqlx::query_as(query)
            .bind(&data.user_id)
            .bind(&data.data)
            .fetch_one(pool)
            .await?;

        Ok(session.0.to_string())
    }

    async fn end_session_postgres(
        &self,
        pool: &Pool<Postgres>,
        data: DeleteSession,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("delete from jen.sessions where id=$1 and user_id=$2")
            .bind(data.id)
            .bind(data.user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    fn start_session_redis() {}

    pub async fn start_session(
        &self,
        pool: &Pool<Postgres>,
        data: CreateSession,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self.interface {
            SessionInterface::Postgres => self.start_session_postgres(pool, data).await,
            SessionInterface::Redis => todo!(),
        }
    }

    fn end_session_redis() {}

    pub fn end_session(&self) {
        match self.interface {
            SessionInterface::Postgres => todo!(),
            SessionInterface::Redis => todo!(),
        }
    }
}
