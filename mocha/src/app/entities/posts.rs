use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::types::{AssetVisibility, PublishStatus};

use super::tags::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub space_id: Uuid,
    pub title: String,
    pub content: String,
    pub read_time: i64,
    pub visibility: AssetVisibility,
    pub status: PublishStatus,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
