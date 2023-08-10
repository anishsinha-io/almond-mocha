use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub image_uri: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSessionById {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSessionsByUserId {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSession {
    pub user_id: String,
    pub data: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditSession {
    pub id: String,
    pub session_state: String,
    pub expires_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSession {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserSessions {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSessionState {
    pub id: String,
    pub session_state: String,
}
