use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::app::{
    dto::auth::{CreateSession, DeleteSession, GetSessionById},
    entities::auth::Session,
    storage::errors::StorageError,
};

pub async fn start_session(
    pool: &Pool<Postgres>,
    data: CreateSession,
) -> Result<String, StorageError> {
    let user_id = Uuid::parse_str(&data.user_id).map_err(|_| {
        log::error!("error converting string to uuid");
        StorageError::PgStartSession
    })?;
    let session = sqlx::query!(
        r#"insert into jen.sessions (user_id, data) values ($1, $2) returning id"#,
        user_id,
        data.data
    )
    .fetch_one(pool)
    .await
    .map_err(|_| StorageError::PgStartSession)?;

    Ok(session.id.to_string())
}

pub async fn end_session(pool: &Pool<Postgres>, data: DeleteSession) -> Result<(), StorageError> {
    let session_id = Uuid::parse_str(&data.id).map_err(|_| {
        log::error!("error converting string (session id) to uuid");
        StorageError::PgEndSession
    })?;
    sqlx::query("delete from jen.sessions where id=$1")
        .bind(session_id)
        .execute(pool)
        .await
        .map_err(|_| StorageError::PgEndSession)?;
    Ok(())
}

pub async fn get_session(
    pool: &Pool<Postgres>,
    data: GetSessionById,
) -> Result<Session, StorageError> {
    let session_id = Uuid::parse_str(&data.id).map_err(|_| {
        log::error!("error converting string (session id) to uuid");
        StorageError::PgEndSession
    })?;
    let session = sqlx::query_as!(
        Session,
        "select id, user_id, data, created_at, updated_at from jen.sessions where id=$1",
        session_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| StorageError::PgGetSession)?;
    Ok(session)
}

//
// pub async fn end_user_sessions(
//     pool: &Pool<Postgres>,
//     data: DeleteUserSessions,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let user_id = Uuid::parse_str(&data.user_id)?;
//     sqlx::query(r#"delete from jen.sessions where user_id=$1"#)
//         .bind(user_id)
//         .execute(pool)
//         .await?;
//     Ok(())
// }
//
// pub async fn update_session_state(
//     pool: &Pool<Postgres>,
//     data: UpdateSessionState,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     sqlx::query(
//         r#"update jen.sessions set session_state=$1 where id=$2 returning id, session_state"#,
//     )
//     .bind(data.session_state)
//     .bind(data.id)
//     .fetch_one(pool)
//     .await?;
//     Ok(())
// }
