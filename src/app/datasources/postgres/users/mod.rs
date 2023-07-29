use sqlx::{Pool, Postgres, Transaction};
use std::error::Error;

use crate::app::dto::CreateUser;

pub async fn create_user(
    pool: &Pool<Postgres>,
    data: CreateUser,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut txn: Transaction<'_, Postgres> = pool.begin().await?;

    let user = sqlx::query!(
        r#"insert into jen.users (first_name, last_name, email, username, image_uri) values ($1, $2, $3, $4, $5) on conflict(email) do nothing returning id"#,
        data.first_name, data.last_name, data.email, data.username,data.image_uri
    )
    .fetch_one(&mut *txn)
    .await?;

    if let (Some(hash), Some(alg)) = (data.hashed_password, data.algorithm) {
        sqlx::query(r#"insert into jen.user_credentials (user_id, credential_hash, alg) values ($1, $2, $3)"#)
            .bind(user.id.to_string())
            .bind(hash)
            .bind(alg)
            .execute(&mut *txn) 
            .await?;
    }

    Ok(user.id.to_string())
}

#[cfg(test) ]
mod tests {
    use crate::app::{dto::HashAlgorithm, datasources::postgres::create_pool};

    use super::*;

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
        });
    }

    pub async fn test_create_user() {
        initialize();
        let pool = create_pool(5).await;

        let new_user = CreateUser{
            first_name: "Jenny".to_owned(),
            last_name: "Sinha".to_owned(),
            email: "jennycho35@gmail.com".to_owned(),
            username: "jennysinha".to_owned(),
            image_uri: "https://assets.anishsinha.com/jenny".to_owned(),
            hashed_password: Some("".to_owned()),
            algorithm: Some(HashAlgorithm::Bcrypt),
        };

        let new_user = create_user(&pool, new_user).await;
    }
}
