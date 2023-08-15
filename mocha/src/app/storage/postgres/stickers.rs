use sqlx::{Executor, Postgres};

use crate::app::dto::stickers::CreateStickerInfo;

pub async fn create_sticker<'a>(
    executor: impl Executor<'a, Database = Postgres>,
    data: CreateStickerInfo,
) {
}
