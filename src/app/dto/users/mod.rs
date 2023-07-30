use serde::{Deserialize, Serialize};

use crate::app::dto::HashAlgorithm;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserById {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserByEmail {
    pub email: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub image_uri: String,
    pub hashed_password: Option<String>,
    pub algorithm: Option<HashAlgorithm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub image_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUser {
    pub id: String,
}
