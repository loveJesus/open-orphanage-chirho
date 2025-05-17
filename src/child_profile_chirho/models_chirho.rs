// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildProfileChirho {
    pub child_id_chirho: String,
    pub orphanage_id_chirho: String,
    pub given_name_chirho: String,
    pub date_of_birth_chirho: DateTime<Utc>,
    pub gender_chirho: String,
    pub medical_history_chirho: String,
    pub education_level_chirho: String,
    pub sponsorship_status_chirho: SponsorshipStatusChirho,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildUpdateChirho {
    pub update_id_chirho: String,
    pub child_id_chirho: String,
    pub staff_user_id_chirho: String,
    pub update_text_chirho: String,
    pub date_posted_chirho: DateTime<Utc>,
    pub visibility_chirho: UpdateVisibilityChirho,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SponsorshipStatusChirho {
    AvailableChirho,
    PendingChirho,
    SponsoredChirho,
}

impl fmt::Display for SponsorshipStatusChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SponsorshipStatusChirho::AvailableChirho => write!(f, "Available"),
            SponsorshipStatusChirho::PendingChirho => write!(f, "Pending"),
            SponsorshipStatusChirho::SponsoredChirho => write!(f, "Sponsored"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UpdateVisibilityChirho {
    PublicChirho,
    PrivateChirho,
    StaffOnlyChirho,
}

impl fmt::Display for UpdateVisibilityChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateVisibilityChirho::PublicChirho => write!(f, "public_chirho"),
            UpdateVisibilityChirho::PrivateChirho => write!(f, "private_chirho"),
            UpdateVisibilityChirho::StaffOnlyChirho => write!(f, "staff_only_chirho"),
        }
    }
}

impl ChildProfileChirho {
    pub fn new(
        orphanage_id_chirho: String,
        given_name_chirho: String,
        date_of_birth_chirho: DateTime<Utc>,
        gender_chirho: String,
        medical_history_chirho: String,
        education_level_chirho: String,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            child_id_chirho: Uuid::new_v4().to_string(),
            orphanage_id_chirho,
            given_name_chirho,
            date_of_birth_chirho,
            gender_chirho,
            medical_history_chirho,
            education_level_chirho,
            sponsorship_status_chirho: SponsorshipStatusChirho::AvailableChirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
}

impl ChildUpdateChirho {
    pub fn new(
        child_id_chirho: String,
        staff_user_id_chirho: String,
        update_text_chirho: String,
        visibility_chirho: UpdateVisibilityChirho,
    ) -> Self {
        Self {
            update_id_chirho: Uuid::new_v4().to_string(),
            child_id_chirho,
            staff_user_id_chirho,
            update_text_chirho,
            date_posted_chirho: Utc::now(),
            visibility_chirho,
        }
    }
} 