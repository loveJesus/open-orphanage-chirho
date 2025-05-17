// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::Env;
use argon2::{
    password_hash::{
        
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use rand_core::OsRng;

use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

use crate::errors_chirho::ErrorChirho;
use super::models_chirho::UserRoleChirho;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsChirho {
    pub sub_chirho: String,
    pub email_chirho: String,
    pub role_chirho: UserRoleChirho,
    pub exp_chirho: i64,
}

pub struct AuthChirho {
    jwt_secret_chirho: String,
}

impl AuthChirho {
    pub fn new(env_chirho: Env) -> Self {
        Self {
            jwt_secret_chirho: env_chirho.secret("JWT_SECRET_CHIRHO").unwrap().to_string(),
        }
    }

    pub fn hash_password_chirho(&self, password_chirho: &str) -> Result<String, ErrorChirho> {
        let salt_chirho = SaltString::generate(&mut OsRng);
        let argon2_chirho = Argon2::default();
        
        let password_hash_chirho = argon2_chirho
            .hash_password(password_chirho.as_bytes(), &salt_chirho)
            .map_err(|e| ErrorChirho::AuthErrorChirho(e.to_string()))?;

        Ok(password_hash_chirho.to_string())
    }

    pub fn verify_password_chirho(&self, password_chirho: &str, hash_chirho: &str) -> Result<bool, ErrorChirho> {
        let parsed_hash_chirho = PasswordHash::new(hash_chirho)
            .map_err(|e| ErrorChirho::AuthErrorChirho(e.to_string()))?;
        
        let argon2_chirho = Argon2::default();
        let is_valid_chirho = argon2_chirho
            .verify_password(password_chirho.as_bytes(), &parsed_hash_chirho)
            .is_ok();

        Ok(is_valid_chirho)
    }

    pub fn generate_token_chirho(&self, user_id_chirho: &str, email_chirho: &str, role_chirho: UserRoleChirho) -> Result<String, ErrorChirho> {
        let expiration_chirho = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims_chirho = ClaimsChirho {
            sub_chirho: user_id_chirho.to_string(),
            email_chirho: email_chirho.to_string(),
            role_chirho,
            exp_chirho: expiration_chirho,
        };

        encode(
            &Header::default(),
            &claims_chirho,
            &EncodingKey::from_secret(self.jwt_secret_chirho.as_bytes()),
        )
        .map_err(|e| ErrorChirho::AuthErrorChirho(e.to_string()))
    }

    pub fn verify_token_chirho(&self, token_chirho: &str) -> Result<ClaimsChirho, ErrorChirho> {
        decode::<ClaimsChirho>(
            token_chirho,
            &DecodingKey::from_secret(self.jwt_secret_chirho.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| ErrorChirho::AuthErrorChirho(e.to_string()))
    }
} 