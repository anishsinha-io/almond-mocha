use sqlx::{Pool, Postgres};
use std::error::Error;
use uuid::Uuid;

use crate::app::dto::CreateSession;

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

    Ok(session.session_state.to_owned())
}
