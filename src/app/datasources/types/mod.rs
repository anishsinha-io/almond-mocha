use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "hash_algorithm", rename_all = "lowercase")]
pub enum HashAlgorithm {
    Argon2,
    Bcrypt,
    Pbkdf2,
    Scrypt,
}
