use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::app::types::{AssetBackend, AssetVisibility};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub visibility: AssetVisibility,
    pub friendly_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub file_path: String,
    pub backend: AssetBackend,
}
