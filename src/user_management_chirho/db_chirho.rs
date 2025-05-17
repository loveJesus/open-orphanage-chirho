// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::*;
use wasm_bindgen::JsValue;

use crate::errors_chirho::ErrorChirho;
use super::models_chirho::{StaffUserChirho, SponsorUserChirho, PlatformAdminChirho};

pub struct UserDbChirho {
    db_chirho: D1Database,
}

impl UserDbChirho {
    pub fn new(env_chirho: Env) -> Self {
        Self {
            db_chirho: env_chirho.d1("DB_CHIRHO").unwrap(),
        }
    }

    pub async fn create_staff_user_chirho(&self, staff_user_chirho: StaffUserChirho) -> Result<()> {
        let query_chirho = r#"
            INSERT INTO staff_users_chirho (
                user_id_chirho,
                email_chirho,
                hashed_password_chirho,
                first_name_chirho,
                last_name_chirho,
                role_chirho,
                orphanage_id_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&staff_user_chirho.user_id_chirho),
                JsValue::from_str(&staff_user_chirho.email_chirho),
                JsValue::from_str(&staff_user_chirho.hashed_password_chirho),
                JsValue::from_str(&staff_user_chirho.first_name_chirho),
                JsValue::from_str(&staff_user_chirho.last_name_chirho),
                JsValue::from_str(&staff_user_chirho.role_chirho.to_string()),
                JsValue::from_str(&staff_user_chirho.orphanage_id_chirho),
                JsValue::from_str(&staff_user_chirho.created_at_chirho.to_rfc3339()),
                JsValue::from_str(&staff_user_chirho.updated_at_chirho.to_rfc3339()),
            ])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn create_sponsor_user_chirho(&self, sponsor_user_chirho: SponsorUserChirho) -> Result<()> {
        let query_chirho = r#"
            INSERT INTO sponsor_users_chirho (
                user_id_chirho,
                email_chirho,
                hashed_password_chirho,
                first_name_chirho,
                last_name_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
        "#;

        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&sponsor_user_chirho.user_id_chirho),
                JsValue::from_str(&sponsor_user_chirho.email_chirho),
                JsValue::from_str(&sponsor_user_chirho.hashed_password_chirho),
                JsValue::from_str(&sponsor_user_chirho.first_name_chirho),
                JsValue::from_str(&sponsor_user_chirho.last_name_chirho),
                JsValue::from_str(&sponsor_user_chirho.created_at_chirho.to_rfc3339()),
                JsValue::from_str(&sponsor_user_chirho.updated_at_chirho.to_rfc3339()),
            ])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn get_staff_user_by_email_chirho(&self, email_chirho: &str) -> Result<Option<StaffUserChirho>> {
        let query_chirho = r#"
            SELECT * FROM staff_users_chirho WHERE email_chirho = ?
        "#;

        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(email_chirho)])?
            .first::<StaffUserChirho>(None)
            .await?;

        Ok(result_chirho)
    }

    pub async fn get_sponsor_user_by_email_chirho(&self, email_chirho: &str) -> Result<Option<SponsorUserChirho>> {
        let query_chirho = r#"
            SELECT * FROM sponsor_users_chirho WHERE email_chirho = ?
        "#;

        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(email_chirho)])?
            .first::<SponsorUserChirho>(None)
            .await?;

        Ok(result_chirho)
    }

    pub async fn get_platform_admin_by_email_chirho(&self, email_chirho: &str) -> Result<Option<PlatformAdminChirho>> {
        let query_chirho = r#"
            SELECT * FROM platform_admins_chirho WHERE email_chirho = ?
        "#;

        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(email_chirho)])?
            .first::<PlatformAdminChirho>(None)
            .await?;

        Ok(result_chirho)
    }
} 