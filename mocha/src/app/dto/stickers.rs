use serde::{Deserialize, Serialize};

use crate::app::types::{AssetBackend, AssetVisibility};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStickerInfo {
    pub visibility: AssetVisibility,
    pub friendly_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSticker {
    pub visibility: AssetVisibility,
    pub friendly_name: String,
    pub file_path: String,
    pub backend: AssetBackend,
}

pub struct CreateStickers {
    pub user_id: String,
    pub stickers: Vec<CreateSticker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStickerById {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStickersByUser {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditStickerInfo {
    pub backend: Option<AssetBackend>,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditSticker {
    pub id: String,
    pub visibility: AssetVisibility,
    pub friendly_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSticker {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAvailableStickers {
    pub user_id: String,
}
