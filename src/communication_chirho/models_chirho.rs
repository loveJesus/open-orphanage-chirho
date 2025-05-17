// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStatusChirho {
    UnreadChirho,
    ReadChirho,
    ArchivedChirho,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChirho {
    pub message_id_chirho: String,
    pub sponsorship_id_chirho: String,
    pub sender_id_chirho: String,
    pub content_chirho: String,
    pub status_chirho: MessageStatusChirho,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

impl MessageChirho {
    pub fn new(
        sponsorship_id_chirho: String,
        sender_id_chirho: String,
        content_chirho: String,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            message_id_chirho: Uuid::new_v4().to_string(),
            sponsorship_id_chirho,
            sender_id_chirho,
            content_chirho,
            status_chirho: MessageStatusChirho::UnreadChirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
} 