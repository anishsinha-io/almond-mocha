use sqlx::{Pool, Postgres};
use std::error::Error;

use crate::app::dto::CreateSpace;

pub async fn create_space(
    pool: Pool<Postgres>,
    data: CreateSpace,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    sqlx::query!(
        r#"insert into jen.spaces (bio, space_name) values ($1, $2)"#,
        data.bio,
        data.space_name
    )
    .execute(&pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::app::datasources::postgres::create_pool;

    use super::*;

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
        });
    }

    #[tokio::test]
    pub async fn test_create_space() {
        initialize();
        let pool = create_pool(5).await;

        let data = CreateSpace {
            space_name: "Computer Science".to_owned(),
            bio: "All things computer science!".to_owned(),
        };

        create_space(pool, data)
            .await
            .expect("error creating new space");
    }
}
