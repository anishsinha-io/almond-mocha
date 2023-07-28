use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub id: Uuid,
    pub space_name: String,
    pub bio: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
