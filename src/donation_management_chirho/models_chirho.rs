// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DonationStatusChirho {
    PendingChirho,
    CompletedChirho,
    FailedChirho,
    RefundedChirho,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DonationChirho {
    pub donation_id_chirho: String,
    pub donor_id_chirho: String,
    pub orphanage_id_chirho: String,
    pub amount_chirho: f64,
    pub status_chirho: DonationStatusChirho,
    pub payment_method_chirho: String,
    pub transaction_id_chirho: Option<String>,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

impl DonationChirho {
    pub fn new(
        donor_id_chirho: String,
        orphanage_id_chirho: String,
        amount_chirho: f64,
        payment_method_chirho: String,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            donation_id_chirho: Uuid::new_v4().to_string(),
            donor_id_chirho,
            orphanage_id_chirho,
            amount_chirho,
            status_chirho: DonationStatusChirho::PendingChirho,
            payment_method_chirho,
            transaction_id_chirho: None,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
} 