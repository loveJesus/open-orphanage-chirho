// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageStatusChirho {
    PendingChirho,
    ApprovedChirho,
    RejectedChirho,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageChirho {
    pub message_id_chirho: String,
    pub sponsorship_id_chirho: String,
    pub sender_user_id_chirho: String,
    pub receiver_child_id_chirho: String,
    pub message_text_chirho: String,
    pub sent_at_chirho: DateTime<Utc>,
    pub read_status_chirho: bool,
    pub moderation_status_chirho: MessageStatusChirho,
    pub moderator_staff_user_id_chirho: Option<String>,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageUpdateChirho {
    pub message_text_chirho: Option<String>,
    pub read_status_chirho: Option<bool>,
    pub moderation_status_chirho: Option<MessageStatusChirho>,
    pub moderator_staff_user_id_chirho: Option<String>,
}

impl MessageChirho {
    pub fn new(
        sponsorship_id_chirho: String,
        sender_user_id_chirho: String,
        receiver_child_id_chirho: String,
        message_text_chirho: String,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            message_id_chirho: Uuid::new_v4().to_string(),
            sponsorship_id_chirho,
            sender_user_id_chirho,
            receiver_child_id_chirho,
            message_text_chirho,
            sent_at_chirho: now_chirho,
            read_status_chirho: false,
            moderation_status_chirho: MessageStatusChirho::PendingChirho,
            moderator_staff_user_id_chirho: None,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }
} 