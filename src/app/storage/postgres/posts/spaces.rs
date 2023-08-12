use chrono::{DateTime, Utc};
use sqlx::{types::Uuid, Pool, Postgres};
use std::error::Error;

use crate::app::dto::{
    CreateSpace, CreateTag, DeleteSpace, DeleteTag, EditSpace, EditTag, GetSpaceById, GetTagById,
    PaginationLimits, SpacePaginationOptions,
};
use crate::app::pagination::PaginationContainer;
use crate::app::storage::entities::{Space, Tag};

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

pub async fn edit_space(
    pool: &Pool<Postgres>,
    data: EditSpace,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let sql = "update jen.spaces set space_name=$2, bio=$3 where id=$1";
    sqlx::query(sql)
        .bind(id)
        .bind(data.space_name)
        .bind(data.bio)
        .execute(pool)
        .await?;
    Ok(())
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

pub async fn get_spaces(
    pool: &Pool<Postgres>,
    pagination: PaginationLimits<SpacePaginationOptions>,
) -> Result<PaginationContainer<Space>, Box<dyn Error + Send + Sync>> {
    let sql = "select id, space_name, bio, created_at, updated_at from jen.spaces
               order by space_name offset $1 limit $2";

    type SpaceTuple = (Uuid, String, String, DateTime<Utc>, DateTime<Utc>);
    let limit = pagination.limit;

    // take one extra
    let rows: Vec<SpaceTuple> = sqlx::query_as(sql)
        .bind(pagination.offset)
        .bind(pagination.limit + 1)
        .fetch_all(pool)
        .await?;

    let all_spaces = rows
        .into_iter()
        .map(|row| Space {
            id: row.0,
            space_name: row.1,
            bio: row.2,
            created_at: row.3,
            updated_at: row.4,
        })
        .collect();

    Ok(PaginationContainer::new(all_spaces, limit))
}

pub async fn create_tag(
    pool: &Pool<Postgres>,
    data: CreateTag,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let sql = "insert into jen.tags (space_id, tag_name, tag_description) values ($1, $2, $3) returning id, tag_name";
    let result: (Uuid, String) = sqlx::query_as(sql)
        .bind(data.space_id)
        .bind(data.name)
        .bind(data.description)
        .fetch_one(pool)
        .await?;
    Ok((result.0.to_string(), result.1))
}

pub async fn get_tag_by_id(
    pool: &Pool<Postgres>,
    data: GetTagById,
) -> Result<Option<Tag>, Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let tag = sqlx::query_as!(
        Tag,
        "select tags.id, tags.tag_name, tags.tag_description, tags.created_at, 
               tags.updated_at, spaces.id as space_id from jen.tags join jen.spaces on 
               spaces.id=tags.space_id and tags.id=$1",
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(tag)
}

pub async fn edit_tag(
    pool: &Pool<Postgres>,
    data: EditTag,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let sql = "update jen.tags set tag_name=$2, tag_description=$3 where id=$1";
    sqlx::query(sql)
        .bind(id)
        .bind(data.name)
        .bind(data.description)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_tag(
    pool: &Pool<Postgres>,
    data: DeleteTag,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let sql = "delete from jen.tags where id=$1";
    sqlx::query(sql).bind(id).execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::app::{storage::postgres, util};

    use super::*;

    #[tokio::test]
    pub async fn test_spaces() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.expect("error creating pool");
        let random_suffix = util::rng::random_string(6);

        let data = CreateSpace {
            space_name: format!("Computer Science [{random_suffix}]"),
            bio: format!("All about computer science [{random_suffix}]"),
        };

        let new_space_id = create_space(&pool, data)
            .await
            .expect("error creating new space");

        let new_space = get_space_by_id(
            &pool,
            GetSpaceById {
                id: new_space_id.clone(),
            },
        )
        .await
        .expect("error fetching new space")
        .expect("space is unexpectedly none");

        log::info!("new space: {:#?}", new_space);

        edit_space(
            &pool,
            EditSpace {
                id: new_space_id.clone(),
                space_name: format!("CS {random_suffix}"),
                bio: format!("A place for computer science {random_suffix}"),
            },
        )
        .await
        .expect("error editing space");

        let edited_space = get_space_by_id(
            &pool,
            GetSpaceById {
                id: new_space_id.clone(),
            },
        )
        .await
        .expect("error fetching space")
        .expect("space is unexpectedly none");

        log::info!("edited space: {:#?}", edited_space);

        delete_space(
            &pool,
            DeleteSpace {
                id: new_space_id.clone(),
            },
        )
        .await
        .expect("error deleting space");

        let deleted_space = get_space_by_id(
            &pool,
            GetSpaceById {
                id: new_space_id.clone(),
            },
        )
        .await
        .expect("error fetching space");

        assert!(deleted_space.is_none());
    }

    #[tokio::test]
    pub async fn test_space_pagination() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.expect("error creating pool");
        let random_suffix = util::rng::random_string(6);

        let mut txn = pool.begin().await.unwrap();

        let new_space_data = vec![
            (
                format!("Computer Science {random_suffix}"),
                format!("A place for CS {random_suffix}"),
            ),
            (
                format!("Math {random_suffix}"),
                format!("A place for math {random_suffix}"),
            ),
            (
                format!("Literature {random_suffix}"),
                format!("A place for literature {random_suffix}"),
            ),
            (
                format!("History {random_suffix}"),
                format!("A place for history {random_suffix}"),
            ),
            (
                format!("Food {random_suffix}"),
                format!("A place for food {random_suffix}"),
            ),
            (
                format!("Travel {random_suffix}"),
                format!("A place for travel {random_suffix}"),
            ),
            (
                format!("Art {random_suffix}"),
                format!("A place for art {random_suffix}"),
            ),
            (
                format!("Fashion {random_suffix}"),
                format!("A place for fashion {random_suffix}"),
            ),
            (
                format!("Chemistry {random_suffix}"),
                format!("A place for chemistry {random_suffix}"),
            ),
            (
                format!("Entertainment {random_suffix}"),
                format!("A place for entertainment {random_suffix}"),
            ),
        ];

        for (name, bio) in new_space_data {
            create_space(
                &pool,
                CreateSpace {
                    space_name: name,
                    bio,
                },
            )
            .await
            .expect("error creating space");
        }

        let spaces = get_spaces(
            &pool,
            PaginationLimits {
                offset: 0,
                limit: 9,
                opts: SpacePaginationOptions { asc: true },
            },
        )
        .await
        .expect("error fetching spaces");

        log::debug!("{:#?}", spaces);

        sqlx::query("delete from jen.spaces where space_name like $1")
            .bind(format!("%{random_suffix}"))
            .execute(&mut *txn)
            .await
            .expect("error clearing table");

        txn.commit().await.expect("error committing txn");
    }
}
