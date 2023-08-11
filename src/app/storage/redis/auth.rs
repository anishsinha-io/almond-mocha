use crate::app::{
    dto::{DeleteSession, GetSessionById},
    storage::{entities::Session, errors::StorageError},
};
use uuid::Uuid;

use crate::app::dto::CreateSession;

use super::{delete, get_json, set_json, RedisConn};

pub async fn start_session(
    conn: &mut RedisConn,
    data: CreateSession,
) -> Result<String, StorageError> {
    let session_id = Uuid::new_v4().to_string();

    set_json(conn, &session_id, data)
        .await
        .map_err(|_| StorageError::RedisStartSession)?;

    Ok(session_id)
}

pub async fn end_session(conn: &mut RedisConn, data: DeleteSession) -> Result<(), StorageError> {
    delete(conn, &data.id)
        .await
        .map_err(|_| StorageError::RedisEndSession)?;
    Ok(())
}

pub async fn get_session(
    conn: &mut RedisConn,
    data: GetSessionById,
) -> Result<Session, StorageError> {
    let session: Option<CreateSession> = get_json(conn, &data.id)
        .await
        .map_err(|_| StorageError::RedisGetSession)?;

    let id = Uuid::parse_str(&data.id).map_err(|e| {
        log::error!("{}", e);
        StorageError::RedisGetSession
    })?;

    match session {
        Some(s) => Ok(Session {
            id,
            user_id: Uuid::parse_str(&s.user_id).map_err(|_| StorageError::RedisGetSession)?,
            data: s.data,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }),
        None => Err(StorageError::NotFound)?,
    }
}
