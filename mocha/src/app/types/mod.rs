use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "jen.hash_algorithm")]
#[sqlx(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Argon2,
    Bcrypt,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy)]
#[sqlx(type_name = "jen.asset_backend")]
#[sqlx(rename_all = "lowercase")]
pub enum AssetBackend {
    Fs,
    Aws,
    Gcp,
    Azure,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy)]
#[sqlx(type_name = "jen.asset_visibility")]
#[sqlx(rename_all = "lowercase")]
pub enum AssetVisibility {
    Public,
    Private,
}
