use serde::{Deserialize, Serialize};

use crate::app::dto::AssetBackend;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSticker {
    pub backend: AssetBackend,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStickerById {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditStickerInfo {
    pub backend: Option<AssetBackend>,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditSticker {
    pub id: String,
    pub backend: Option<AssetBackend>,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSticker {
    pub id: String,
}

