use sqlx::{Acquire, Executor, Postgres};
use std::error::Error;
use uuid::Uuid;

use crate::app::{
    dto::stickers::{
        CreateStickers, DeleteSticker, EditSticker, GetStickerById, GetStickersByUser,
    },
    entities::stickers::Sticker,
    types::{AssetBackend, AssetVisibility},
};

pub async fn create_stickers<'a>(
    executor: impl Executor<'a, Database = Postgres> + Acquire<'a, Database = Postgres>,
    data: CreateStickers,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let mut txn = executor.begin().await?;
    let mut inserted_ids = Vec::<String>::new();
    let user_id = Uuid::parse_str(&data.user_id)?;
    for sticker in data.stickers {
        let asset_query =
            "insert into jen.assets (backend, file_path) values ($1, $2) returning id";
        let sticker_query = "insert into jen.stickers (user_id, asset_id, visibility, friendly_name) values ($1, $2, $3, $4) returning id";

        let (asset_id,): (Uuid,) = sqlx::query_as(asset_query)
            .bind(sticker.backend)
            .bind(sticker.file_path)
            .fetch_one(&mut *txn)
            .await?;
        let (sticker_id,): (Uuid,) = sqlx::query_as(sticker_query)
            .bind(user_id)
            .bind(asset_id)
            .bind(sticker.visibility)
            .bind(sticker.friendly_name)
            .fetch_one(&mut *txn)
            .await?;
        inserted_ids.push(sticker_id.to_string());
    }

    txn.commit().await?;
    Ok(inserted_ids)
}

pub async fn get_sticker<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetStickerById,
) -> Result<Sticker, Box<dyn Error + Send + Sync>> {
    let sticker_id = Uuid::parse_str(&data.id)?;
    let sticker: Sticker = sqlx::query_as!(
        Sticker,
        r#"select stickers.id, user_id, asset_id, visibility as "visibility!: AssetVisibility", 
           friendly_name, stickers.created_at, stickers.updated_at, assets.file_path, 
           assets.backend as "backend!: AssetBackend" from jen.stickers join jen.assets on 
           jen.stickers.asset_id=jen.assets.id and jen.stickers.id=$1"#,
        sticker_id
    )
    .fetch_one(executor)
    .await?;

    Ok(sticker)
}

pub async fn get_stickers_by_user<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: GetStickersByUser,
) -> Result<Vec<Sticker>, Box<dyn Error + Send + Sync>> {
    let user_id = Uuid::parse_str(&data.user_id)?;
    let stickers: Vec<Sticker> = sqlx::query_as!(
        Sticker,
        r#"select stickers.id, user_id, asset_id, visibility as "visibility!: AssetVisibility", 
           friendly_name, stickers.created_at, stickers.updated_at, assets.file_path, 
           assets.backend as "backend!: AssetBackend" from jen.stickers join jen.assets on 
           jen.stickers.asset_id=jen.assets.id and user_id=$1"#,
        user_id
    )
    .fetch_all(executor)
    .await?;
    Ok(stickers)
}

pub async fn edit_sticker<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: EditSticker,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let sticker_id = Uuid::parse_str(&data.id)?;
    let sql = "update jen.stickers set visibility=$2, friendly_name=$3 where id=$1";
    sqlx::query(sql)
        .bind(sticker_id)
        .bind(data.visibility)
        .bind(&data.friendly_name)
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn delete_sticker<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: DeleteSticker,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = Uuid::parse_str(&data.id)?;
    let sql = "delete from jen.stickers where id=$1";
    sqlx::query(sql).bind(id).execute(executor).await?;
    Ok(())
}
