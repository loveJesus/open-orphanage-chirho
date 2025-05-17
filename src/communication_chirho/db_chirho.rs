// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

// Database access for communication_chirho module. 

use worker::*;
use super::models_chirho::{MessageChirho, MessageStatusChirho};

pub struct MessageDbChirho {
    db_chirho: D1Database,
}

impl MessageDbChirho {
    pub fn new(env_chirho: Env) -> Self {
        Self {
            db_chirho: env_chirho.d1("DB_CHIRHO").unwrap(),
        }
    }

    pub async fn create_message_chirho(&self, message_chirho: MessageChirho) -> Result<()> {
        let query_chirho = r#"
            INSERT INTO messages_chirho (
                message_id_chirho,
                sponsorship_id_chirho,
                sender_user_id_chirho,
                receiver_child_id_chirho,
                message_text_chirho,
                sent_at_chirho,
                read_status_chirho,
                moderation_status_chirho,
                moderator_staff_user_id_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                message_chirho.message_id_chirho.clone().into(),
                message_chirho.sponsorship_id_chirho.clone().into(),
                message_chirho.sender_user_id_chirho.clone().into(),
                message_chirho.receiver_child_id_chirho.clone().into(),
                message_chirho.message_text_chirho.clone().into(),
                message_chirho.sent_at_chirho.to_rfc3339().into(),
                message_chirho.read_status_chirho.into(),
                format!("{:?}", message_chirho.moderation_status_chirho).into(),
                message_chirho.moderator_staff_user_id_chirho.clone().unwrap_or_default().into(),
                message_chirho.created_at_chirho.to_rfc3339().into(),
                message_chirho.updated_at_chirho.to_rfc3339().into(),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_message_chirho(&self, message_id_chirho: &str) -> Result<Option<MessageChirho>> {
        let query_chirho = r#"
            SELECT * FROM messages_chirho WHERE message_id_chirho = ?
        "#;

        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[message_id_chirho.into()])?
            .first::<MessageChirho>(None)
            .await?;

        Ok(result_chirho)
    }

    pub async fn update_message_status_chirho(&self, message_id_chirho: &str, status_chirho: MessageStatusChirho) -> Result<()> {
        let query_chirho = r#"
            UPDATE messages_chirho SET status_chirho = ?, updated_at_chirho = ? WHERE message_id_chirho = ?
        "#;

        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                format!("{:?}", status_chirho).into(),
                chrono::Utc::now().to_rfc3339().into(),
                message_id_chirho.into(),
            ])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn get_messages_by_sponsorship_chirho(&self, sponsorship_id_chirho: &str) -> Result<Vec<MessageChirho>> {
        let query_chirho = r#"
            SELECT * FROM messages_chirho WHERE sponsorship_id_chirho = ?
        "#;

        let d1_result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[sponsorship_id_chirho.into()])?
            .all()
            .await?;

        let messages_chirho: Vec<MessageChirho> = d1_result_chirho.results()?;
        Ok(messages_chirho)
    }
} 