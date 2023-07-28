use sqlx::{types::Uuid, Pool, Postgres};
use std::error::Error;

use crate::app::datasources::entities::Space;
use crate::app::dto::{CreateSpace, GetSpaceById};

pub async fn create_space(
    pool: &Pool<Postgres>,
    data: CreateSpace,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let space = sqlx::query!(
        r#"insert into jen.spaces (bio, space_name) values ($1, $2) on conflict (space_name) do update set bio=$2 returning id"#,
        data.bio,
        data.space_name
    )
    .fetch_one(pool)
    .await?;

    Ok(space.id.to_string())
}

pub async fn get_space_by_id(
    pool: &Pool<Postgres>,
    data: GetSpaceById,
) -> Result<Option<Space>, Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id).unwrap_or_else(|_| Uuid::nil());
    let space: Option<Space> = sqlx::query_as!(
        Space,
        r#"select id, space_name, bio, created_at, updated_at from jen.spaces where id=$1"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(space)
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
    pub async fn test_create_and_get_space() {
        initialize();
        let pool = create_pool(5).await;

        let data = CreateSpace {
            space_name: "Computer Science".to_owned(),
            bio: "All things computer science!".to_owned(),
        };

        let id = create_space(&pool, data)
            .await
            .expect("error creating new space");

        let get_space_data = GetSpaceById { id };

        let space = get_space_by_id(&pool, get_space_data)
            .await
            .expect("error fetching space")
            .unwrap();

        println!("{:#?}", space);
    }
}
