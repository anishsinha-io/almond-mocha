use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub space_id: Uuid,
    pub tag_name: String,
    pub tag_description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
