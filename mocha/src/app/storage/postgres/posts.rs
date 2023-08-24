use std::error::Error;

use sqlx::{Acquire, Executor, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::app::dto::posts::{CreateOrSaveDraft, GetPostById};

pub async fn create_or_save_draft<'a>(
    executor: impl Executor<'a, Database = Postgres> + Acquire<'a, Database = Postgres>,
    data: CreateOrSaveDraft,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user_id = Uuid::parse_str(&data.user_id)?;
    let space_id = Uuid::parse_str(&data.space_id)?;
    let mut txn = executor.begin().await?;
    match data.id {
        Some(id) => {
            let sql = "update jen.posts set space_id=$1, title=$2, 
                       content=$3, visibility=$4, read_time=$5 where 
                       id=$6 and user_id=$7";

            let post_id = Uuid::parse_str(&id)?;
            sqlx::query(sql)
                .bind(space_id)
                .bind(data.title)
                .bind(data.content)
                .bind(data.visibility)
                .bind(data.read_time)
                .bind(post_id)
                .bind(user_id)
                .execute(&mut *txn)
                .await?;

            let mut builder: QueryBuilder<Postgres> =
                QueryBuilder::new("insert into jen.post_tags (post_id, tag_id) on conflict (post_id, tag_id) do nothing");

            builder.push_values(data.tags.into_iter(), |mut b, t| {
                if let Ok(tag_id) = Uuid::parse_str(&t) {
                    b.push_bind(post_id).push_bind(tag_id);
                }
            });
            let query = builder.build();
            query.execute(&mut *txn).await?;
            Ok(())
        }
        None => {
            let sql = "insert into jen.posts (user_id, space_id, title, content, visibility, read_time) values ($1, $2, $3, $4, $5, $6) returning id";
            let (post_id,): (Uuid,) = sqlx::query_as(sql)
                .bind(user_id)
                .bind(space_id)
                .bind(data.title)
                .bind(data.content)
                .bind(data.visibility)
                .bind(data.read_time)
                .fetch_one(&mut *txn)
                .await?;

            let mut builder: QueryBuilder<Postgres> =
                QueryBuilder::new("insert into jen.post_tags (post_id, tag_id) on conflict (post_id, tag_id) do nothing");

            builder.push_values(data.tags.into_iter(), |mut b, t| {
                if let Ok(tag_id) = Uuid::parse_str(&t) {
                    b.push_bind(post_id).push_bind(tag_id);
                }
            });
            let query = builder.build();
            query.execute(&mut *txn).await?;
            Ok(())
        }
    }
}

pub async fn get_post_by_id<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetPostById,
) {
    let sql = "select ";
}

pub async fn get_user_posts() {}

pub async fn get_user_archived_posts() {}

pub async fn get_user_drafts() {}

pub async fn edit_post() {}

pub async fn delete_post() {}
