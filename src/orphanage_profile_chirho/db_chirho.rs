// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::D1Database;
use wasm_bindgen::JsValue;

use crate::errors_chirho::ErrorChirho;
use super::models_chirho::{OrphanageProfileChirho, OrphanageVerificationChirho};

pub struct OrphanageDbChirho {
    pub db_chirho: D1Database,
}

impl OrphanageDbChirho {
    pub fn new(db_chirho: D1Database) -> Self {
        Self { db_chirho }
    }

    pub async fn create_orphanage_profile_chirho(&self, orphanage_profile_chirho: OrphanageProfileChirho) -> Result<OrphanageProfileChirho, ErrorChirho> {
        let query_chirho = r#"
            INSERT INTO orphanage_profiles_chirho (
                orphanage_id_chirho,
                name_chirho,
                description_chirho,
                address_chirho,
                contact_email_chirho,
                contact_phone_chirho,
                website_url_chirho,
                capacity_chirho,
                current_children_count_chirho,
                founded_date_chirho,
                registration_number_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&orphanage_profile_chirho.orphanage_id_chirho),
                JsValue::from_str(&orphanage_profile_chirho.name_chirho),
                JsValue::from_str(&orphanage_profile_chirho.description_chirho),
                JsValue::from_str(&orphanage_profile_chirho.address_chirho),
                JsValue::from_str(&orphanage_profile_chirho.contact_email_chirho),
                JsValue::from_str(&orphanage_profile_chirho.contact_phone_chirho),
                JsValue::from_str(orphanage_profile_chirho.website_url_chirho.as_deref().unwrap_or("")),
                JsValue::from_str(&orphanage_profile_chirho.capacity_chirho.to_string()),
                JsValue::from_str(&orphanage_profile_chirho.current_children_count_chirho.to_string()),
                JsValue::from_str(&orphanage_profile_chirho.founded_date_chirho.to_rfc3339()),
                JsValue::from_str(&orphanage_profile_chirho.registration_number_chirho),
                JsValue::from_str(&orphanage_profile_chirho.created_at_chirho.to_rfc3339()),
                JsValue::from_str(&orphanage_profile_chirho.updated_at_chirho.to_rfc3339()),
            ])?
            .run()
            .await?;
        Ok(orphanage_profile_chirho)
    }

    pub async fn get_orphanage_profile_chirho(&self, orphanage_id_chirho: &str) -> Result<Option<OrphanageProfileChirho>, ErrorChirho> {
        let query_chirho = r#"
            SELECT * FROM orphanage_profiles_chirho WHERE orphanage_id_chirho = ?
        "#;
        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(orphanage_id_chirho)])?
            .first::<OrphanageProfileChirho>(None)
            .await?;
        Ok(result_chirho)
    }

    pub async fn update_orphanage_profile_chirho(&self, orphanage_profile_chirho: OrphanageProfileChirho) -> Result<OrphanageProfileChirho, ErrorChirho> {
        let query_chirho = r#"
            UPDATE orphanage_profiles_chirho SET
                name_chirho = ?,
                description_chirho = ?,
                address_chirho = ?,
                contact_email_chirho = ?,
                contact_phone_chirho = ?,
                website_url_chirho = ?,
                capacity_chirho = ?,
                current_children_count_chirho = ?,
                founded_date_chirho = ?,
                registration_number_chirho = ?,
                updated_at_chirho = ?
            WHERE orphanage_id_chirho = ?
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&orphanage_profile_chirho.name_chirho),
                JsValue::from_str(&orphanage_profile_chirho.description_chirho),
                JsValue::from_str(&orphanage_profile_chirho.address_chirho),
                JsValue::from_str(&orphanage_profile_chirho.contact_email_chirho),
                JsValue::from_str(&orphanage_profile_chirho.contact_phone_chirho),
                JsValue::from_str(orphanage_profile_chirho.website_url_chirho.as_deref().unwrap_or("")),
                JsValue::from_str(&orphanage_profile_chirho.capacity_chirho.to_string()),
                JsValue::from_str(&orphanage_profile_chirho.current_children_count_chirho.to_string()),
                JsValue::from_str(&orphanage_profile_chirho.founded_date_chirho.to_rfc3339()),
                JsValue::from_str(&orphanage_profile_chirho.registration_number_chirho),
                JsValue::from_str(&orphanage_profile_chirho.updated_at_chirho.to_rfc3339()),
                JsValue::from_str(&orphanage_profile_chirho.orphanage_id_chirho),
            ])?
            .run()
            .await?;
        Ok(orphanage_profile_chirho)
    }

    pub async fn delete_orphanage_profile_chirho(&self, orphanage_id_chirho: &str) -> Result<(), ErrorChirho> {
        let query_chirho = r#"
            DELETE FROM orphanage_profiles_chirho WHERE orphanage_id_chirho = ?
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(orphanage_id_chirho)])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn verify_orphanage_chirho(&self, verification_chirho: OrphanageVerificationChirho) -> Result<(), ErrorChirho> {
        let query_chirho = r#"
            UPDATE orphanage_profiles_chirho 
            SET is_verified_chirho = true,
                verification_date_chirho = ?,
                verified_by_chirho = ?
            WHERE orphanage_id_chirho = ?
        "#;

        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&verification_chirho.verification_date_chirho.to_rfc3339()),
                JsValue::from_str(&verification_chirho.verified_by_chirho),
                JsValue::from_str(&verification_chirho.orphanage_id_chirho),
            ])?
            .run()
            .await?;

        Ok(())
    }
} 