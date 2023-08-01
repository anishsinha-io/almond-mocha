use serde::{Deserialize, Serialize};

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
    pub session_state: String,
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
    pub user_id: String,
    pub session_state: String,
}
