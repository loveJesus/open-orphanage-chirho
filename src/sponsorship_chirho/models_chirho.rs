// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SponsorshipChirho {
    pub sponsorship_id_chirho: String,
    pub sponsor_id_chirho: String,
    pub child_id_chirho: String,
    pub monthly_amount_chirho: f64,
    pub start_date_chirho: DateTime<Utc>,
    pub end_date_chirho: Option<DateTime<Utc>>,
    pub status_chirho: SponsorshipStatusChirho,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SponsorshipStatusChirho {
    ActiveChirho,
    PausedChirho,
    EndedChirho,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SponsorshipUpdateChirho {
    pub monthly_amount_chirho: Option<f64>,
    pub end_date_chirho: Option<DateTime<Utc>>,
    pub status_chirho: Option<SponsorshipStatusChirho>,
}

impl SponsorshipChirho {
    pub fn new(
        sponsor_id_chirho: String,
        child_id_chirho: String,
        monthly_amount_chirho: f64,
        start_date_chirho: DateTime<Utc>,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            sponsorship_id_chirho: Uuid::new_v4().to_string(),
            sponsor_id_chirho,
            child_id_chirho,
            monthly_amount_chirho,
            start_date_chirho,
            end_date_chirho: None,
            status_chirho: SponsorshipStatusChirho::ActiveChirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
} 