use crate::app::dto::HashAlgorithm;
use argon2::{self, Config};
use rand::Rng;
use std::error::Error;

pub struct CredentialManager {
    pub algorithm: HashAlgorithm,
}

impl CredentialManager {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        Self { algorithm }
    }

    fn gen_random_bytes(&self) -> [u8; 16] {
        rand::thread_rng().gen::<[u8; 16]>()
    }

    fn hash_argon2(&self, candidate: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>> {
        let salt = self.gen_random_bytes();
        let config = Config::default();
        let hash = argon2::hash_encoded(candidate, &salt, &config)?;
        Ok(hash)
    }

    fn hash_bcrypt(&self, candidate: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>> {
        let salt = self.gen_random_bytes();
        let hash_result = bcrypt::hash_with_salt(candidate, 10, salt)?;
        Ok(hash_result.to_string())
    }

    pub fn create_hash(&self, candidate: &[u8]) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self.algorithm {
            HashAlgorithm::Argon2 => self.hash_argon2(candidate),
            HashAlgorithm::Bcrypt => self.hash_bcrypt(candidate),
        }
    }

    pub fn verify_hash(&self, candidate: &str, hash: &str) -> bool {
        match self.algorithm {
            HashAlgorithm::Argon2 => {
                argon2::verify_encoded(hash, candidate.as_bytes()).unwrap_or(false)
            }
            HashAlgorithm::Bcrypt => bcrypt::verify(candidate, hash).unwrap_or(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_credential_manager() {
        let plaintext = "jennysinha";

        let mut manager = CredentialManager {
            algorithm: HashAlgorithm::Argon2,
        };

        let argon2_hash = manager.create_hash(plaintext.as_bytes()).unwrap();

        let mut correct = manager.verify_hash(plaintext, &argon2_hash);
        assert!(correct);

        let mut incorrect = manager.verify_hash("incorrect_password", &argon2_hash);
        assert!(!incorrect);

        manager.algorithm = HashAlgorithm::Bcrypt;

        let bcrypt_hash = manager.create_hash(plaintext.as_bytes()).unwrap();

        correct = manager.verify_hash(plaintext, &bcrypt_hash);
        assert!(correct);

        incorrect = manager.verify_hash("incorrect_password", &bcrypt_hash);
        assert!(!incorrect);
    }
}
