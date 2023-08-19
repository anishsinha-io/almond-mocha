use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use uuid::Uuid;

use crate::app::config::StorageLayer;
use crate::app::dto::auth::GetUserRbac;
use crate::app::entities::auth::UserRbac;
use crate::app::storage::postgres;
use crate::app::util;

pub static ISS: &str = "milkandmocha";
pub static AUD: &str = "milkandmocha";
pub static ACCESS_TOKEN_LIFETIME: usize = 60 * 5;
// pub static REFRESH_TOKEN_LIFETIME: usize = 60 * 60 * 24 * 30 * 3;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub jti: String,
    pub iat: usize,
    pub exp: usize,
    pub nbf: usize,
    pub rbac: UserRbac,
}

impl Claims {
    pub fn sign_rs256(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let private_key = env::var("RSA_PRIVATE_KEY")?;
        let token = jsonwebtoken::encode(
            &Header::new(Algorithm::RS256),
            self,
            &EncodingKey::from_rsa_pem(private_key.as_bytes())?,
        )?;
        Ok(token)
    }

    pub async fn new_signed(
        storage_layer: &StorageLayer,
        sub: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let rbac = postgres::auth::get_user_rbac(
            &storage_layer.pg,
            GetUserRbac {
                user_id: sub.to_owned(),
            },
        )
        .await?;

        let jti = Uuid::new_v4().to_string();
        let iat = util::time::now();

        let nbf = iat;
        let exp = iat + ACCESS_TOKEN_LIFETIME;

        let claims = Claims {
            sub: sub.to_owned(),
            iss: ISS.to_owned(),
            aud: AUD.to_owned(),
            jti,
            iat,
            nbf,
            exp,
            rbac,
        };

        let token = claims.sign_rs256()?;

        Ok(token)
    }
}

pub fn verify_rs256(token: &str) -> Result<TokenData<Claims>, Box<dyn Error + Send + Sync>> {
    let public_key = env::var("RSA_PUBLIC_KEY")?;
    let decoded = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(public_key.as_bytes())?,
        &Validation::new(Algorithm::RS256),
    )?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        time::{SystemTime, UNIX_EPOCH},
    };

    use uuid::Uuid;

    use super::*;

    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
        });
    }

    #[test]
    fn test_sign_and_verify() {
        initialize();
        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let nbf = iat;
        let exp = iat + ACCESS_TOKEN_LIFETIME;

        let jti = Uuid::new_v4().to_string();

        let rbac = UserRbac {
            role_membership: vec![],
            permissions: vec![],
        };

        let claims = Claims {
            sub: "sub".to_owned(),
            iss: ISS.to_owned(),
            aud: AUD.to_owned(),
            jti,
            iat,
            exp,
            nbf,
            rbac,
        };

        let _signed = claims.sign_rs256().unwrap();

        // println!("{signed}");
        let _verified_header = verify_rs256(&_signed).unwrap().header;
        // println!("{:#?}", _verified_header);
        let _verified_claims = verify_rs256(&_signed).unwrap().claims;
        // println!("{:#?}", _verified_claims);
    }
}
