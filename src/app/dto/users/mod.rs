use serde::{Deserialize, Serialize};

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
    pub image_uri: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub image_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUser {
    pub id: String,
}
