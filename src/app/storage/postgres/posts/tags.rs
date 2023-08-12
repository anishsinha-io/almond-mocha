use sqlx::{types::Uuid, Pool, Postgres};
use std::error::Error;

use crate::app::{
    dto::{CreateTag, DeleteTag, EditTag, GetTagById},
    storage::entities::Tag,
};

pub async fn create_tag(
    pool: &Pool<Postgres>,
    data: CreateTag,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let query =
        "insert into jen.tags (tag_name, tag_description) values ($1, $2) returning id, tag_name";
    let res: (Uuid, String) = sqlx::query_as(query)
        .bind(data.name)
        .bind(data.description)
        .fetch_one(pool)
        .await?;

    Ok((res.0.to_string(), res.1))
}

pub async fn get_tag_by_id(
    pool: &Pool<Postgres>,
    data: GetTagById,
) -> Result<Option<Tag>, Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let res = sqlx::query_as!(
        Tag,
        "select id, tag_name, tag_description, created_at, updated_at from jen.tags where id=$1",
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(res)
}

pub async fn edit_tag(
    pool: &Pool<Postgres>,
    data: EditTag,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let query = "update jen.tags set tag_name=$2, tag_description=$3 where id=$1";
    sqlx::query(query)
        .bind(data.id)
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
    let query = "delete from jen.tags where id=$1";
    sqlx::query(query).bind(data.id).execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::app::storage::postgres::create_pool;

    use super::*;

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
        });
    }

    #[tokio::test]
    pub async fn test_tags() {
        initialize();
        let pool = create_pool(5)
            .await
            .expect("error creating connection pool");

        let (new_tag_id, new_tag_name) = create_tag(
            &pool,
            CreateTag {
                name: "rust".to_owned(),
                description: "Posts about the rust programming language".to_owned(),
            },
        )
        .await
        .expect("error creating tag");

        println!("new tag created: {new_tag_name}");

        let tag = get_tag_by_id(
            &pool,
            GetTagById {
                id: new_tag_id.clone(),
            },
        )
        .await
        .expect("error fetching tag")
        .expect("tag is none");

        println!("{:#?}", tag);

        edit_tag(
            &pool,
            EditTag {
                id: tag.id.to_string(),
                name: "rust-lang".to_owned(),
                description: "All about the Rust programming language".to_owned(),
            },
        )
        .await
        .expect("error editing tag");

        let edited_tag = get_tag_by_id(
            &pool,
            GetTagById {
                id: new_tag_id.clone(),
            },
        )
        .await
        .expect("error fetching tag")
        .expect("tag is none");

        assert_eq!(edited_tag.tag_name, "rust-lang");

        delete_tag(
            &pool,
            DeleteTag {
                id: new_tag_id.to_owned(),
            },
        )
        .await
        .expect("error deleting tag");
    }
}
