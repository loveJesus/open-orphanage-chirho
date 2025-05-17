// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRoleChirho {
    AdminChirho,
    StaffChirho,
    SponsorChirho,
}

impl fmt::Display for UserRoleChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRoleChirho::AdminChirho => write!(f, "admin_chirho"),
            UserRoleChirho::StaffChirho => write!(f, "staff_chirho"),
            UserRoleChirho::SponsorChirho => write!(f, "sponsor_chirho"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StaffUserChirho {
    pub user_id_chirho: String,
    pub email_chirho: String,
    pub hashed_password_chirho: String,
    pub first_name_chirho: String,
    pub last_name_chirho: String,
    pub role_chirho: UserRoleChirho,
    pub orphanage_id_chirho: String,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SponsorUserChirho {
    pub user_id_chirho: String,
    pub email_chirho: String,
    pub hashed_password_chirho: String,
    pub first_name_chirho: String,
    pub last_name_chirho: String,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlatformAdminChirho {
    pub user_id_chirho: String,
    pub email_chirho: String,
    pub hashed_password_chirho: String,
    pub first_name_chirho: String,
    pub last_name_chirho: String,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequestChirho {
    pub email_chirho: String,
    pub password_chirho: String,
    pub full_name_chirho: String,
    pub role_chirho: Option<UserRoleChirho>,
    pub orphanage_id_chirho: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestChirho {
    pub email_chirho: String,
    pub password_chirho: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseChirho {
    pub token_chirho: String,
    pub user_id_chirho: String,
    pub role_chirho: UserRoleChirho,
}

impl StaffUserChirho {
    pub fn new(
        orphanage_id_chirho: String,
        email_chirho: String,
        hashed_password_chirho: String,
        first_name_chirho: String,
        last_name_chirho: String,
        role_chirho: UserRoleChirho,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            user_id_chirho: Uuid::new_v4().to_string(),
            email_chirho,
            hashed_password_chirho,
            first_name_chirho,
            last_name_chirho,
            role_chirho,
            orphanage_id_chirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
}

impl SponsorUserChirho {
    pub fn new(
        email_chirho: String,
        hashed_password_chirho: String,
        first_name_chirho: String,
        last_name_chirho: String,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            user_id_chirho: Uuid::new_v4().to_string(),
            email_chirho,
            hashed_password_chirho,
            first_name_chirho,
            last_name_chirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
}

impl PlatformAdminChirho {
    pub fn new(
        email_chirho: String,
        hashed_password_chirho: String,
        first_name_chirho: String,
        last_name_chirho: String,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            user_id_chirho: Uuid::new_v4().to_string(),
            email_chirho,
            hashed_password_chirho,
            first_name_chirho,
            last_name_chirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
} 