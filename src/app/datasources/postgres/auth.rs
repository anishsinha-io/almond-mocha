use sqlx::{Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

use crate::app::dto::{CreateSession, DeleteUserSessions, UpdateSessionState};

pub async fn start_session(
    pool: &Pool<Postgres>,
    data: CreateSession,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let user_id = Uuid::parse_str(&data.user_id)?;
    let session = sqlx::query!(
        r#"insert into jen.sessions (user_id, session_state) values ($1, $2) returning session_state"#,
        user_id,
        data.session_state
    ).fetch_one(pool).await?;

    Ok(session.session_state)
}

pub async fn end_user_sessions(
    pool: &Pool<Postgres>,
    data: DeleteUserSessions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user_id = Uuid::parse_str(&data.user_id)?;
    sqlx::query(r#"delete from jen.sessions where user_id=$1"#)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_session_state(
    pool: &Pool<Postgres>,
    data: UpdateSessionState,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    sqlx::query(
        r#"update jen.sessions set session_state=$1 where id=$2 returning id, session_state"#,
    )
    .bind(data.session_state)
    .bind(data.id)
    .fetch_one(pool)
    .await?;
    Ok(())
}
