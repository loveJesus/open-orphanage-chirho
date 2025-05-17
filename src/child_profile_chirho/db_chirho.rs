// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::{D1Database};
use wasm_bindgen::JsValue;
use chrono::{DateTime, Utc};

use crate::errors_chirho::ErrorChirho;
use super::models_chirho::{ChildProfileChirho, ChildUpdateChirho};

pub struct ChildDbChirho {
    pub db_chirho: D1Database,
}

impl ChildDbChirho {
    pub fn new(db_chirho: D1Database) -> Self {
        Self { db_chirho }
    }

    pub async fn create_child_profile_chirho(&self, child_profile_chirho: ChildProfileChirho) -> Result<ChildProfileChirho, ErrorChirho> {
        let query_chirho = r#"
            INSERT INTO child_profiles_chirho (
                child_id_chirho,
                orphanage_id_chirho,
                given_name_chirho,
                date_of_birth_chirho,
                gender_chirho,
                medical_history_chirho,
                education_level_chirho,
                sponsorship_status_chirho,
                created_at_chirho,
                updated_at_chirho
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&child_profile_chirho.child_id_chirho),
                JsValue::from_str(&child_profile_chirho.orphanage_id_chirho),
                JsValue::from_str(&child_profile_chirho.given_name_chirho),
                JsValue::from_str(&child_profile_chirho.date_of_birth_chirho.to_rfc3339()),
                JsValue::from_str(&child_profile_chirho.gender_chirho),
                JsValue::from_str(&child_profile_chirho.medical_history_chirho),
                JsValue::from_str(&child_profile_chirho.education_level_chirho),
                JsValue::from_str(&child_profile_chirho.sponsorship_status_chirho.to_string()),
                JsValue::from_str(&child_profile_chirho.created_at_chirho.to_rfc3339()),
                JsValue::from_str(&child_profile_chirho.updated_at_chirho.to_rfc3339()),
            ])?
            .run()
            .await?;
        Ok(child_profile_chirho)
    }

    pub async fn get_child_profile_chirho(&self, child_id_chirho: &str) -> Result<Option<ChildProfileChirho>, ErrorChirho> {
        let query_chirho = r#"
            SELECT * FROM child_profiles_chirho WHERE child_id_chirho = ?
        "#;
        let result_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(child_id_chirho)])?
            .first::<ChildProfileChirho>(None)
            .await?;
        Ok(result_chirho)
    }

    pub async fn get_children_by_orphanage_chirho(&self, orphanage_id_chirho: &str) -> Result<Vec<ChildProfileChirho>, ErrorChirho> {
        let query_chirho = r#"
            SELECT * FROM child_profiles_chirho WHERE orphanage_id_chirho = ?
        "#;
        let results_chirho = self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(orphanage_id_chirho)])?
            .all()
            .await?;
        // Convert D1Result to Vec<ChildProfileChirho>
        let children_chirho: Vec<ChildProfileChirho> = results_chirho.results::<ChildProfileChirho>()?;
        Ok(children_chirho)
    }

    pub async fn update_child_profile_chirho(&self, child_profile_chirho: ChildProfileChirho) -> Result<ChildProfileChirho, ErrorChirho> {
        let query_chirho = r#"
            UPDATE child_profiles_chirho SET
                given_name_chirho = ?,
                date_of_birth_chirho = ?,
                gender_chirho = ?,
                medical_history_chirho = ?,
                education_level_chirho = ?,
                sponsorship_status_chirho = ?,
                updated_at_chirho = ?
            WHERE child_id_chirho = ?
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&child_profile_chirho.given_name_chirho),
                JsValue::from_str(&child_profile_chirho.date_of_birth_chirho.to_rfc3339()),
                JsValue::from_str(&child_profile_chirho.gender_chirho),
                JsValue::from_str(&child_profile_chirho.medical_history_chirho),
                JsValue::from_str(&child_profile_chirho.education_level_chirho),
                JsValue::from_str(&child_profile_chirho.sponsorship_status_chirho.to_string()),
                JsValue::from_str(&child_profile_chirho.updated_at_chirho.to_rfc3339()),
                JsValue::from_str(&child_profile_chirho.child_id_chirho),
            ])?
            .run()
            .await?;
        Ok(child_profile_chirho)
    }

    pub async fn delete_child_profile_chirho(&self, child_id_chirho: &str) -> Result<(), ErrorChirho> {
        let query_chirho = r#"
            DELETE FROM child_profiles_chirho WHERE child_id_chirho = ?
        "#;
        self.db_chirho
            .prepare(query_chirho)
            .bind(&[JsValue::from_str(child_id_chirho)])?
            .run()
            .await?;
        Ok(())
    }

    pub async fn create_child_update_chirho(&self, update_chirho: ChildUpdateChirho) -> Result<(), ErrorChirho> {
        let query_chirho = r#"
            INSERT INTO child_updates_chirho (
                update_id_chirho, child_id_chirho, staff_user_id_chirho,
                update_text_chirho, date_posted_chirho, visibility_chirho
            ) VALUES (?, ?, ?, ?, ?, ?)
        "#;

        self.db_chirho
            .prepare(query_chirho)
            .bind(&[
                JsValue::from_str(&update_chirho.update_id_chirho),
                JsValue::from_str(&update_chirho.child_id_chirho),
                JsValue::from_str(&update_chirho.staff_user_id_chirho),
                JsValue::from_str(&update_chirho.update_text_chirho),
                JsValue::from_str(&update_chirho.date_posted_chirho.to_rfc3339()),
                JsValue::from_str(&update_chirho.visibility_chirho.to_string()),
            ])?
            .run()
            .await?;

        Ok(())
    }
} 