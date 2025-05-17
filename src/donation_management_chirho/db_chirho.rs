// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::*;
use super::models_chirho::{DonationChirho, DonationStatusChirho};
use std::fmt;

impl fmt::Display for DonationStatusChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DonationStatusChirho::PendingChirho => write!(f, "pending_chirho"),
            DonationStatusChirho::CompletedChirho => write!(f, "completed_chirho"),
            DonationStatusChirho::FailedChirho => write!(f, "failed_chirho"),
            DonationStatusChirho::RefundedChirho => write!(f, "refunded_chirho"),
        }
    }
}

pub struct DonationDbChirho {
    db_chirho: D1Database,
}

impl DonationDbChirho {
    pub fn new(env_chirho: Env) -> Self {
        Self {
            db_chirho: env_chirho.d1("DB_CHIRHO").unwrap(),
        }
    }

    pub async fn create_donation_chirho(&self, donation_chirho: DonationChirho) -> Result<()> {
        let query_chirho = r#"
            INSERT INTO donations_chirho (
                donation_id_chirho,
                donor_id_chirho,
                orphanage_id_chirho,
                amount_chirho,
                status_chirho,
                payment_method_chirho,
                transaction_id_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                donation_chirho.donation_id_chirho.into(),
                donation_chirho.donor_id_chirho.into(),
                donation_chirho.orphanage_id_chirho.into(),
                donation_chirho.amount_chirho.into(),
                donation_chirho.status_chirho.to_string().into(),
                donation_chirho.payment_method_chirho.into(),
                donation_chirho.transaction_id_chirho.unwrap_or_default().into(),
                donation_chirho.created_at_chirho.to_rfc3339().into(),
                donation_chirho.updated_at_chirho.to_rfc3339().into(),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_donation_chirho(&self, donation_id_chirho: &str) -> Result<Option<DonationChirho>> {
        let query_chirho = r#"
            SELECT * FROM donations_chirho WHERE donation_id_chirho = ?
        "#;
        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[donation_id_chirho.into()])?
            .first::<DonationChirho>(None)
            .await?;
        Ok(result_chirho)
    }

    pub async fn update_donation_status_chirho(&self, donation_id_chirho: &str, status_chirho: DonationStatusChirho) -> Result<()> {
        let query_chirho = r#"
            UPDATE donations_chirho SET status_chirho = ?, updated_at_chirho = ? WHERE donation_id_chirho = ?
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                status_chirho.to_string().into(),
                chrono::Utc::now().to_rfc3339().into(),
                donation_id_chirho.into(),
            ])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_donor_donations_chirho(&self, donor_id_chirho: &str) -> Result<Vec<DonationChirho>> {
        let query_chirho = r#"
            SELECT * FROM donations_chirho WHERE donor_id_chirho = ?
        "#;
        let results_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[donor_id_chirho.into()])?
            .all()
            .await?;
        let donations_chirho: Vec<DonationChirho> = results_chirho.results()?;
        Ok(donations_chirho)
    }

    pub async fn get_orphanage_donations_chirho(&self, orphanage_id_chirho: &str) -> Result<Vec<DonationChirho>> {
        let query_chirho = r#"
            SELECT * FROM donations_chirho WHERE orphanage_id_chirho = ?
        "#;
        let results_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[orphanage_id_chirho.into()])?
            .all()
            .await?;
        let donations_chirho: Vec<DonationChirho> = results_chirho.results()?;
        Ok(donations_chirho)
    }
} 