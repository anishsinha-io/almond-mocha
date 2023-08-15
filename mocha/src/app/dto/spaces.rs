use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpace {
    pub space_name: String,
    pub bio: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSpaceById {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSpace {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditSpaceInfo {
    pub space_name: String,
    pub bio: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditSpace {
    pub id: String,
    pub space_name: String,
    pub bio: String,
}
