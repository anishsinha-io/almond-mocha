use sqlx::{Pool, Postgres};

use crate::app::dto::CreateSession;

// pub async fn start_session(pool: &Pool<Postgres>, data: CreateSession) -> Result<(String, String), Box<dyn Error+Send+Sync>>{
//     let id_and_state = sqlx::query!(r#"insert into jen.sessions"#);
// }
