use serde::{Deserialize, Serialize};

use crate::app::types::{AssetVisibility, PublishStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct EditStickerReq {
    pub friendly_name: String,
    pub visibility: AssetVisibility,
}

pub struct CreateOrSaveDraftReq {
    pub id: Option<String>,
    pub space_id: String,
    pub title: String,
    pub content: String,
    pub read_time: i64,
    pub visibility: AssetVisibility,
    pub tags: Vec<String>,
    pub status: PublishStatus,
}
