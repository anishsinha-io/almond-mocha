use chrono::{DateTime, Utc};
use sqlx::{types::Uuid, Postgres};
use sqlx::{Executor, QueryBuilder};
use std::error::Error;

use crate::app::dto::{
    CreateSpace, CreateTag, DeleteSpace, DeleteTag, EditSpace, EditTag, GetSpaceById, GetTagById,
    GetTagsBySpace, PaginationLimits, SpacePaginationOptions, TagPaginationOptions,
};
use crate::app::pagination::PaginationContainer;
use crate::app::storage::entities::{Space, Tag};

pub async fn create_space<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: CreateSpace,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let space = sqlx::query!(
        r#"insert into jen.spaces (bio, space_name) values ($1, $2) on conflict (space_name) do update set bio=$1 returning id"#,
        data.bio,
        data.space_name
    )
    .fetch_one(executor)
    .await?;

    Ok(space.id.to_string())
}

pub async fn get_space_by_id<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetSpaceById,
) -> Result<Option<Space>, Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let space: Option<Space> = sqlx::query_as!(
        Space,
        r#"select id, space_name, bio, created_at, updated_at from jen.spaces where id=$1"#,
        id
    )
    .fetch_optional(executor)
    .await?;
    Ok(space)
}

pub async fn edit_space<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: EditSpace,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let sql = "update jen.spaces set space_name=$2, bio=$3 where id=$1";
    sqlx::query(sql)
        .bind(id)
        .bind(data.space_name)
        .bind(data.bio)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn delete_space<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: DeleteSpace,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    sqlx::query!(r#"delete from jen.spaces where id=$1"#, id)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn get_spaces<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    pagination: PaginationLimits<SpacePaginationOptions>,
) -> Result<PaginationContainer<Space>, Box<dyn Error + Send + Sync>> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "select id, space_name, bio, created_at, updated_at from jen.spaces order by space_name ",
    );
    if pagination.opts.asc {
        query_builder.push("asc");
    } else {
        query_builder.push("desc");
    };
    let sql = query_builder
        .push(" offset ")
        .push_bind(pagination.offset)
        .push(" limit ")
        .push_bind(pagination.limit + 1)
        .build_query_as();

    type SpaceTuple = (Uuid, String, String, DateTime<Utc>, DateTime<Utc>);
    let limit = pagination.limit;

    let rows: Vec<SpaceTuple> = sql.fetch_all(executor).await?;

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

pub async fn create_tag<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: CreateTag,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let sql = "insert into jen.tags (space_id, tag_name, tag_description) values ($1, $2, $3) returning id, tag_name";
    let space_id = Uuid::parse_str(&data.space_id)?;
    let result: (Uuid, String) = sqlx::query_as(sql)
        .bind(space_id)
        .bind(data.name)
        .bind(data.description)
        .fetch_one(executor)
        .await?;
    Ok((result.0.to_string(), result.1))
}

pub async fn get_tag_by_id<'a>(
    executor: impl Executor<'a, Database = Postgres>,
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
    .fetch_optional(executor)
    .await?;
    Ok(tag)
}

pub async fn get_tags<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetTagsBySpace,
    pagination: PaginationLimits<TagPaginationOptions>,
) -> Result<PaginationContainer<Tag>, Box<dyn Error + Send + Sync>> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "select id, space_id, tag_name, tag_description, created_at, updated_at from jen.tags where space_id=",
    );
    let space_id = Uuid::parse_str(&data.space_id)?;
    query_builder
        .push_bind(space_id)
        .push(" order by tag_name ");

    if pagination.opts.asc {
        query_builder.push(" asc ");
    } else {
        query_builder.push(" desc ");
    };

    let sql = query_builder
        .push(" limit ")
        .push_bind(pagination.limit + 1)
        .push(" offset ")
        .push_bind(pagination.offset)
        .build_query_as();

    type TagTuple = (Uuid, Uuid, String, String, DateTime<Utc>, DateTime<Utc>);
    let limit = pagination.limit;

    let rows: Vec<TagTuple> = sql.fetch_all(executor).await?;

    let tags = rows
        .into_iter()
        .map(|row| Tag {
            id: row.0,
            space_id: row.1,
            tag_name: row.2,
            tag_description: row.3,
            created_at: row.4,
            updated_at: row.5,
        })
        .collect();

    Ok(PaginationContainer::new(tags, limit))
}

pub async fn edit_tag<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: EditTag,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let sql = "update jen.tags set tag_name=$2, tag_description=$3 where id=$1";
    sqlx::query(sql)
        .bind(id)
        .bind(data.name)
        .bind(data.description)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn delete_tag<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: DeleteTag,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let sql = "delete from jen.tags where id=$1";
    sqlx::query(sql).bind(id).execute(executor).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::app::{storage::postgres, util};
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    pub async fn test_spaces() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.expect("error creating pool");
        let random_suffix = util::rng::random_string(6);

        let mut txn = pool.begin().await.expect("error starting transaction");

        let data = CreateSpace {
            space_name: format!("Computer Science [{random_suffix}]"),
            bio: format!("All about computer science [{random_suffix}]"),
        };

        let new_space_id = create_space(&mut *txn, data)
            .await
            .expect("error creating new space");

        let new_space = get_space_by_id(
            &mut *txn,
            GetSpaceById {
                id: new_space_id.clone(),
            },
        )
        .await
        .expect("error fetching new space")
        .expect("space is unexpectedly none");

        log::info!("new space: {:#?}", new_space);

        edit_space(
            &mut *txn,
            EditSpace {
                id: new_space_id.clone(),
                space_name: format!("CS {random_suffix}"),
                bio: format!("A place for computer science {random_suffix}"),
            },
        )
        .await
        .expect("error editing space");

        let edited_space = get_space_by_id(
            &mut *txn,
            GetSpaceById {
                id: new_space_id.clone(),
            },
        )
        .await
        .expect("error fetching space")
        .expect("space is unexpectedly none");

        log::info!("edited space: {:#?}", edited_space);

        delete_space(
            &mut *txn,
            DeleteSpace {
                id: new_space_id.clone(),
            },
        )
        .await
        .expect("error deleting space");

        let deleted_space = get_space_by_id(
            &mut *txn,
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
                &mut *txn,
                CreateSpace {
                    space_name: name,
                    bio,
                },
            )
            .await
            .expect("error creating space");
        }

        let spaces = get_spaces(
            &mut *txn,
            PaginationLimits {
                offset: 0,
                limit: 9,
                opts: SpacePaginationOptions { asc: false },
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

        txn.rollback().await.expect("error committing txn");
    }

    #[tokio::test]
    pub async fn test_tags() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.expect("error creating pool");
        let random_suffix = util::rng::random_string(6);

        let mut txn = pool.begin().await.unwrap();

        let music_space = create_space(
            &mut *txn,
            CreateSpace {
                space_name: format!("Music {random_suffix}"),
                bio: format!("All about music {random_suffix}"),
            },
        )
        .await
        .expect("error creating space");

        let (new_tag_id, new_tag_name) = create_tag(
            &mut *txn,
            CreateTag {
                space_id: music_space.to_owned(),
                name: format!("Debussy {random_suffix}"),
                description: format!("All about Debussy's music {random_suffix}"),
            },
        )
        .await
        .expect("error creating tag");

        log::info!("{new_tag_name}");

        let tag = get_tag_by_id(
            &mut *txn,
            GetTagById {
                id: new_tag_id.clone(),
            },
        )
        .await
        .expect("error fetching tag")
        .expect("tag is unexpectedly none");

        log::info!("{:#?}", tag);

        edit_tag(
            &mut *txn,
            EditTag {
                id: tag.id.clone().to_string(),
                name: format!("music {random_suffix}"),
                description: format!("a place for music {random_suffix}"),
            },
        )
        .await
        .expect("error editing tag");

        let edited_tag = get_tag_by_id(
            &mut *txn,
            GetTagById {
                id: new_tag_id.clone(),
            },
        )
        .await
        .expect("error fetching tag")
        .expect("tag is unexpectedly none");

        log::info!("{:#?}", edited_tag);

        delete_tag(
            &mut *txn,
            DeleteTag {
                id: tag.id.clone().to_string(),
            },
        )
        .await
        .expect("error deleting tag");

        txn.rollback()
            .await
            .expect("error rolling back transaction");
    }

    #[tokio::test]
    pub async fn test_tags_pagination() {
        util::test_util::init();
        let pool = postgres::create_pool(5).await.expect("error creating pool");
        let random_suffix = util::rng::random_string(6);

        let mut txn = pool.begin().await.unwrap();

        let cs_space = create_space(
            &mut *txn,
            CreateSpace {
                space_name: format!("Computer Science {random_suffix}"),
                bio: format!("A place for computer science {random_suffix}"),
            },
        )
        .await
        .expect("error creating computer science space");

        let tag_data = vec![
            (
                format!("rust {random_suffix}"),
                format!("all about the rust programming language {random_suffix}"),
            ),
            (
                format!("c++ {random_suffix}"),
                format!("all about the c++ programming language {random_suffix}"),
            ),
            (
                format!("python {random_suffix}"),
                format!("all about the python programming language {random_suffix}"),
            ),
            (
                format!("typescript {random_suffix}"),
                format!("all about the typescript programming language {random_suffix}"),
            ),
            (
                format!("java {random_suffix}"),
                format!("all about the java programming language {random_suffix}"),
            ),
            (
                format!("kotlin {random_suffix}"),
                format!("all about the kotlin programming language {random_suffix}"),
            ),
            (
                format!("clojure {random_suffix}"),
                format!("all about the clojure programming language {random_suffix}"),
            ),
            (
                format!("elixir {random_suffix}"),
                format!("all about the elixir programming language {random_suffix}"),
            ),
            (
                format!("ruby {random_suffix}"),
                format!("all about the ruby programming language {random_suffix}"),
            ),
            (
                format!("go {random_suffix}"),
                format!("all about the go programming language {random_suffix}"),
            ),
        ];

        let mut new_tag_ids = HashMap::<String, String>::new();

        for (name, description) in tag_data {
            let (tag_id, tag_name) = create_tag(
                &mut *txn,
                CreateTag {
                    space_id: cs_space.clone(),
                    name,
                    description,
                },
            )
            .await
            .expect("error creating space");
            new_tag_ids.insert(tag_name, tag_id);
        }

        let tags_container = get_tags(
            &mut *txn,
            GetTagsBySpace { space_id: cs_space },
            PaginationLimits {
                offset: 0,
                limit: 10,
                opts: TagPaginationOptions { asc: false },
            },
        )
        .await
        .expect("error getting tags");

        assert!(tags_container.done);

        txn.rollback()
            .await
            .expect("error rolling back transaction")
    }
}
