use serde::{Deserialize, Serialize};

use crate::app::types::AssetBackend;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStickerInfo {
    pub private: bool,
    pub friendly_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSticker {
    pub private: bool,
    pub friendly_name: String,
    pub file_path: String,
    pub backend: AssetBackend,
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
