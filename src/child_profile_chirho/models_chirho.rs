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
    pub given_name_chirho: Option<String>,
    pub date_of_birth_chirho: Option<DateTime<Utc>>,
    pub gender_chirho: Option<String>,
    pub medical_history_chirho: Option<String>,
    pub education_level_chirho: Option<String>,
    pub sponsorship_status_chirho: Option<SponsorshipStatusChirho>,
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

    pub fn update(&mut self, update_chirho: ChildUpdateChirho) {
        if let Some(given_name_chirho) = update_chirho.given_name_chirho {
            self.given_name_chirho = given_name_chirho;
        }
        if let Some(date_of_birth_chirho) = update_chirho.date_of_birth_chirho {
            self.date_of_birth_chirho = date_of_birth_chirho;
        }
        if let Some(gender_chirho) = update_chirho.gender_chirho {
            self.gender_chirho = gender_chirho;
        }
        if let Some(medical_history_chirho) = update_chirho.medical_history_chirho {
            self.medical_history_chirho = medical_history_chirho;
        }
        if let Some(education_level_chirho) = update_chirho.education_level_chirho {
            self.education_level_chirho = education_level_chirho;
        }
        if let Some(sponsorship_status_chirho) = update_chirho.sponsorship_status_chirho {
            self.sponsorship_status_chirho = sponsorship_status_chirho;
        }
        self.updated_at_chirho = Utc::now();
    }
} 