use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
