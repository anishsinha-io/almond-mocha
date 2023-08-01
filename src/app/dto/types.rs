use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "jen.hash_algorithm")]
#[sqlx(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Argon2,
    Bcrypt,
}
