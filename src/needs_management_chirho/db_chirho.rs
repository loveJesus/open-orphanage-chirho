// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::*;
use super::models_chirho::{NeedChirho, NeedUpdateChirho, NeedStatusChirho};
use std::fmt;

impl fmt::Display for NeedStatusChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeedStatusChirho::ActiveChirho => write!(f, "active_chirho"),
            NeedStatusChirho::FulfilledChirho => write!(f, "fulfilled_chirho"),
            NeedStatusChirho::CancelledChirho => write!(f, "cancelled_chirho"),
        }
    }
}

pub struct NeedDbChirho {
    db_chirho: D1Database,
}

impl NeedDbChirho {
    pub fn new(env_chirho: Env) -> Self {
        Self {
            db_chirho: env_chirho.d1("DB_CHIRHO").unwrap(),
        }
    }

    pub async fn create_need_chirho(&self, need_chirho: NeedChirho) -> Result<()> {
        let query_chirho = r#"
            INSERT INTO needs_chirho (
                need_id_chirho,
                orphanage_id_chirho,
                title_chirho,
                description_chirho,
                amount_needed_chirho,
                amount_raised_chirho,
                status_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                need_chirho.need_id_chirho.into(),
                need_chirho.orphanage_id_chirho.into(),
                need_chirho.title_chirho.into(),
                need_chirho.description_chirho.into(),
                need_chirho.amount_needed_chirho.into(),
                need_chirho.amount_raised_chirho.into(),
                need_chirho.status_chirho.to_string().into(),
                need_chirho.created_at_chirho.to_rfc3339().into(),
                need_chirho.updated_at_chirho.to_rfc3339().into(),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_need_chirho(&self, need_id_chirho: &str) -> Result<Option<NeedChirho>> {
        let query_chirho = r#"
            SELECT * FROM needs_chirho WHERE need_id_chirho = ?
        "#;
        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[need_id_chirho.into()])?
            .first::<NeedChirho>(None)
            .await?;
        Ok(result_chirho)
    }

    pub async fn update_need_chirho(&self, need_id_chirho: &str, update_chirho: NeedUpdateChirho) -> Result<()> {
        // Build dynamic SQL and params
        let mut sets_chirho = Vec::new();
        let mut params_chirho: Vec<String> = Vec::new();
        if let Some(title) = update_chirho.title_chirho {
            sets_chirho.push("title_chirho = ?");
            params_chirho.push(title);
        }
        if let Some(description) = update_chirho.description_chirho {
            sets_chirho.push("description_chirho = ?");
            params_chirho.push(description);
        }
        if let Some(amount_needed) = update_chirho.amount_needed_chirho {
            sets_chirho.push("amount_needed_chirho = ?");
            params_chirho.push(amount_needed.to_string());
        }
        if let Some(amount_raised) = update_chirho.amount_raised_chirho {
            sets_chirho.push("amount_raised_chirho = ?");
            params_chirho.push(amount_raised.to_string());
        }
        if let Some(status) = update_chirho.status_chirho {
            sets_chirho.push("status_chirho = ?");
            params_chirho.push(status.to_string());
        }
        sets_chirho.push("updated_at_chirho = ?");
        params_chirho.push(chrono::Utc::now().to_rfc3339());
        params_chirho.push(need_id_chirho.to_string());
        let set_clause_chirho = sets_chirho.join(", ");
        let query_chirho = format!(
            "UPDATE needs_chirho SET {} WHERE need_id_chirho = ?",
            set_clause_chirho
        );
        self.db_chirho
            .prepare(&query_chirho)
            .bind(&params_chirho.iter().map(|s| s.into()).collect::<Vec<_>>())?
            .run()
            .await?;
        Ok(())
    }

    pub async fn delete_need_chirho(&self, need_id_chirho: &str) -> Result<()> {
        let query_chirho = r#"
            DELETE FROM needs_chirho WHERE need_id_chirho = ?
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[need_id_chirho.into()])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_orphanage_needs_chirho(&self, orphanage_id_chirho: &str) -> Result<Vec<NeedChirho>> {
        let query_chirho = r#"
            SELECT * FROM needs_chirho WHERE orphanage_id_chirho = ?
        "#;
        let results_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[orphanage_id_chirho.into()])?
            .all()
            .await?;
        let needs_chirho: Vec<NeedChirho> = results_chirho.results()?;
        Ok(needs_chirho)
    }
} 