// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::*;
use super::models_chirho::{SponsorshipChirho, SponsorshipStatusChirho, SponsorshipUpdateChirho};
use std::fmt;

impl fmt::Display for SponsorshipStatusChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SponsorshipStatusChirho::ActiveChirho => write!(f, "active_chirho"),
            SponsorshipStatusChirho::PausedChirho => write!(f, "paused_chirho"),
            SponsorshipStatusChirho::EndedChirho => write!(f, "ended_chirho"),
        }
    }
}

pub struct SponsorshipDbChirho {
    db_chirho: D1Database,
}

impl SponsorshipDbChirho {
    pub fn new(env_chirho: Env) -> Self {
        Self {
            db_chirho: env_chirho.d1("DB_CHIRHO").unwrap(),
        }
    }

    pub async fn create_sponsorship_chirho(&self, sponsorship_chirho: SponsorshipChirho) -> Result<()> {
        let query_chirho = r#"
            INSERT INTO sponsorships_chirho (
                sponsorship_id_chirho,
                sponsor_id_chirho,
                child_id_chirho,
                monthly_amount_chirho,
                start_date_chirho,
                end_date_chirho,
                status_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                sponsorship_chirho.sponsorship_id_chirho.clone().into(),
                sponsorship_chirho.sponsor_id_chirho.clone().into(),
                sponsorship_chirho.child_id_chirho.clone().into(),
                sponsorship_chirho.monthly_amount_chirho.into(),
                sponsorship_chirho.start_date_chirho.to_rfc3339().into(),
                sponsorship_chirho.end_date_chirho.map(|d| d.to_rfc3339()).unwrap_or_default().into(),
                sponsorship_chirho.status_chirho.to_string().into(),
                sponsorship_chirho.created_at_chirho.to_rfc3339().into(),
                sponsorship_chirho.updated_at_chirho.to_rfc3339().into(),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_sponsorship_chirho(&self, sponsorship_id_chirho: &str) -> Result<Option<SponsorshipChirho>> {
        let query_chirho = r#"
            SELECT * FROM sponsorships_chirho WHERE sponsorship_id_chirho = ?
        "#;
        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[sponsorship_id_chirho.into()])?
            .first::<SponsorshipChirho>(None)
            .await?;
        Ok(result_chirho)
    }

    pub async fn update_sponsorship_chirho(&self, sponsorship_id_chirho: &str, update_chirho: SponsorshipUpdateChirho) -> Result<()> {
        let mut sets_chirho = Vec::new();
        let mut params_chirho: Vec<String> = Vec::new();
        if let Some(monthly_amount) = update_chirho.monthly_amount_chirho {
            sets_chirho.push("monthly_amount_chirho = ?");
            params_chirho.push(monthly_amount.to_string());
        }
        if let Some(end_date) = update_chirho.end_date_chirho {
            sets_chirho.push("end_date_chirho = ?");
            params_chirho.push(end_date.to_rfc3339());
        }
        if let Some(status) = update_chirho.status_chirho {
            sets_chirho.push("status_chirho = ?");
            params_chirho.push(status.to_string());
        }
        sets_chirho.push("updated_at_chirho = ?");
        params_chirho.push(chrono::Utc::now().to_rfc3339());
        params_chirho.push(sponsorship_id_chirho.to_string());
        let set_clause_chirho = sets_chirho.join(", ");
        let query_chirho = format!(
            "UPDATE sponsorships_chirho SET {} WHERE sponsorship_id_chirho = ?",
            set_clause_chirho
        );
        self.db_chirho
            .prepare(&query_chirho)
            .bind(&params_chirho.iter().map(|s| s.into()).collect::<Vec<_>>())?
            .run()
            .await?;
        Ok(())
    }

    pub async fn delete_sponsorship_chirho(&self, sponsorship_id_chirho: &str) -> Result<()> {
        let query_chirho = r#"
            DELETE FROM sponsorships_chirho WHERE sponsorship_id_chirho = ?
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[sponsorship_id_chirho.into()])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_sponsor_sponsorships_chirho(&self, sponsor_id_chirho: &str) -> Result<Vec<SponsorshipChirho>> {
        let query_chirho = r#"
            SELECT * FROM sponsorships_chirho WHERE sponsor_id_chirho = ?
        "#;
        let results_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[sponsor_id_chirho.into()])?
            .all()
            .await?;
        let sponsorships_chirho: Vec<SponsorshipChirho> = results_chirho.results()?;
        Ok(sponsorships_chirho)
    }

    pub async fn get_child_sponsorships_chirho(&self, child_id_chirho: &str) -> Result<Vec<SponsorshipChirho>> {
        let query_chirho = r#"
            SELECT * FROM sponsorships_chirho WHERE child_id_chirho = ?
        "#;
        let results_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[child_id_chirho.into()])?
            .all()
            .await?;
        let sponsorships_chirho: Vec<SponsorshipChirho> = results_chirho.results()?;
        Ok(sponsorships_chirho)
    }
} 