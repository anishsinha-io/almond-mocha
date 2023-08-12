use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTagById {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTagByName {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditTag {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTag {
    pub id: String,
}
