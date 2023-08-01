use sqlx::{types::Uuid, Pool, Postgres};
use std::error::Error;

use crate::app::datasources::entities::Space;
use crate::app::dto::{CreateSpace, DeleteSpace, GetSpaceById, PaginationLimits};
use crate::app::pagination::PaginationContainer;

pub async fn create_space(
    pool: &Pool<Postgres>,
    data: CreateSpace,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let space = sqlx::query!(
        r#"insert into jen.spaces (bio, space_name) values ($1, $2) on conflict (space_name) do update set bio=$1 returning id"#,
        data.bio,
        data.space_name
    )
    .fetch_one(pool)
    .await?;

    Ok(space.id.to_string())
}

pub async fn get_spaces(
    pool: &Pool<Postgres>,
    data: PaginationLimits,
) -> Result<PaginationContainer<Space>, Box<dyn Error + Send + Sync>> {
    let spaces: Vec<Space> = sqlx::query_as!(
        Space,
        r#"select id, space_name, bio, created_at, updated_at from jen.spaces order by space_name limit $1 offset $2"#,
        (data.limit as i64) + 1,
        data.offset as i64
    )
    .fetch_all(pool)
    .await?;

    let num_items = spaces.len() - 1;

    let container = PaginationContainer {
        items: spaces[..num_items].to_vec(),
        start: data.offset,
        end: data.offset + data.limit,
        done: (data.limit as usize) > num_items,
    };
    Ok(container)
}

pub async fn get_space_by_id(
    pool: &Pool<Postgres>,
    data: GetSpaceById,
) -> Result<Option<Space>, Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let space: Option<Space> = sqlx::query_as!(
        Space,
        r#"select id, space_name, bio, created_at, updated_at from jen.spaces where id=$1"#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(space)
}

pub async fn delete_space(
    pool: &Pool<Postgres>,
    data: DeleteSpace,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    sqlx::query!(r#"delete from jen.spaces where id=$1"#, id)
        .execute(pool)
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
    pub async fn test_spaces() {
        initialize();
        let pool = create_pool(5).await.unwrap();

        let data = CreateSpace {
            space_name: "Computer Science".to_owned(),
            bio: "All things computer science!".to_owned(),
        };

        let id = create_space(&pool, data)
            .await
            .expect("error creating new space");

        let get_space_data = GetSpaceById { id: id.clone() };

        let valid_space = get_space_by_id(&pool, get_space_data)
            .await
            .expect("error fetching space");

        assert!(valid_space.is_some());

        let get_invalid_space_data = GetSpaceById {
            id: "invalid".to_owned(),
        };

        get_space_by_id(&pool, get_invalid_space_data)
            .await
            .expect_err("space is invalid");

        let new_spaces = [
            ("Mathematics", "All about math."),
            ("Biology", "Posts about biology"),
            ("Chemistry", "All about chemistry!"),
            ("Physics", "Everything about physics (:"),
        ];

        for space in new_spaces {
            create_space(
                &pool,
                CreateSpace {
                    space_name: space.0.to_owned(),
                    bio: space.1.to_owned(),
                },
            )
            .await
            .expect("error creating space");
        }

        let mut limits = PaginationLimits {
            offset: 0,
            limit: 5,
        };

        let mut spaces = get_spaces(&pool, limits.clone())
            .await
            .expect("error getting spaces");

        assert!(spaces.done);

        limits.limit = 4;
        spaces = get_spaces(&pool, limits.clone())
            .await
            .expect("error getting spaces");
        assert!(!spaces.done);

        limits.limit = 6;
        spaces = get_spaces(&pool, limits.clone())
            .await
            .expect("error getting spaces");
        assert!(spaces.done);

        delete_space(&pool, DeleteSpace { id: id.clone() })
            .await
            .expect("error deleting space");

        let deleted_space = get_space_by_id(&pool, GetSpaceById { id: id.clone() })
            .await
            .expect("error getting space");
        assert!(deleted_space.is_none());
    }
}
