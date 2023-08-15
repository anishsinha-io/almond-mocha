use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::app::types::AssetBackend;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    pub id: Uuid,
    pub user_id: Uuid,
    pub private: bool,
    pub backend: AssetBackend,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
