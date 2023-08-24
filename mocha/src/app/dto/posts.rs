use serde::{Deserialize, Serialize};

use crate::app::types::AssetVisibility;

#[derive(Serialize, Deserialize)]
pub struct GetPostById {
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPostsByUser {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrSaveDraft {
    pub id: Option<String>,
    pub user_id: String,
    pub space_id: String,
    pub title: String,
    pub content: String,
    pub visibility: AssetVisibility,
    pub tags: Vec<String>,
    pub read_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishPost {
    pub user_id: String,
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivePost {
    pub user_id: String,
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditPost {
    pub user_id: String,
    pub post_id: String,
    pub title: String,
    pub private: bool,
    pub tags: Vec<String>,
    pub content: String,
    pub image_uri: String,
    pub read_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePost {
    pub user_id: String,
    pub post_id: String,
}
