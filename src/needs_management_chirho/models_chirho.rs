// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NeedStatusChirho {
    ActiveChirho,
    FulfilledChirho,
    CancelledChirho,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NeedChirho {
    pub need_id_chirho: String,
    pub orphanage_id_chirho: String,
    pub title_chirho: String,
    pub description_chirho: String,
    pub amount_needed_chirho: f64,
    pub amount_raised_chirho: f64,
    pub status_chirho: NeedStatusChirho,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

impl NeedChirho {
    pub fn new(
        orphanage_id_chirho: String,
        title_chirho: String,
        description_chirho: String,
        amount_needed_chirho: f64,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            need_id_chirho: Uuid::new_v4().to_string(),
            orphanage_id_chirho,
            title_chirho,
            description_chirho,
            amount_needed_chirho,
            amount_raised_chirho: 0.0,
            status_chirho: NeedStatusChirho::ActiveChirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NeedUpdateChirho {
    pub title_chirho: Option<String>,
    pub description_chirho: Option<String>,
    pub amount_needed_chirho: Option<f64>,
    pub amount_raised_chirho: Option<f64>,
    pub status_chirho: Option<NeedStatusChirho>,
} 