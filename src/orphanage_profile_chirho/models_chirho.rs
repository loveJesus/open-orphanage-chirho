// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrphanageProfileChirho {
    pub orphanage_id_chirho: String,
    pub name_chirho: String,
    pub description_chirho: String,
    pub address_chirho: String,
    pub contact_email_chirho: String,
    pub contact_phone_chirho: String,
    pub website_url_chirho: Option<String>,
    pub capacity_chirho: i32,
    pub current_children_count_chirho: i32,
    pub founded_date_chirho: DateTime<Utc>,
    pub registration_number_chirho: String,
    pub created_at_chirho: DateTime<Utc>,
    pub updated_at_chirho: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrphanageUpdateChirho {
    pub name_chirho: Option<String>,
    pub description_chirho: Option<String>,
    pub address_chirho: Option<String>,
    pub contact_email_chirho: Option<String>,
    pub contact_phone_chirho: Option<String>,
    pub website_url_chirho: Option<String>,
    pub capacity_chirho: Option<i32>,
    pub current_children_count_chirho: Option<i32>,
    pub founded_date_chirho: Option<DateTime<Utc>>,
    pub registration_number_chirho: Option<String>,
}

impl OrphanageProfileChirho {
    pub fn new(
        name_chirho: String,
        description_chirho: String,
        address_chirho: String,
        contact_email_chirho: String,
        contact_phone_chirho: String,
        website_url_chirho: Option<String>,
        capacity_chirho: i32,
        founded_date_chirho: DateTime<Utc>,
        registration_number_chirho: String,
    ) -> Self {
        let now_chirho = Utc::now();
        Self {
            orphanage_id_chirho: Uuid::new_v4().to_string(),
            name_chirho,
            description_chirho,
            address_chirho,
            contact_email_chirho,
            contact_phone_chirho,
            website_url_chirho,
            capacity_chirho,
            current_children_count_chirho: 0,
            founded_date_chirho,
            registration_number_chirho,
            created_at_chirho: now_chirho,
            updated_at_chirho: now_chirho,
        }
    }

    pub fn update(&mut self, update_chirho: OrphanageUpdateChirho) {
        if let Some(name_chirho) = update_chirho.name_chirho {
            self.name_chirho = name_chirho;
        }
        if let Some(description_chirho) = update_chirho.description_chirho {
            self.description_chirho = description_chirho;
        }
        if let Some(address_chirho) = update_chirho.address_chirho {
            self.address_chirho = address_chirho;
        }
        if let Some(contact_email_chirho) = update_chirho.contact_email_chirho {
            self.contact_email_chirho = contact_email_chirho;
        }
        if let Some(contact_phone_chirho) = update_chirho.contact_phone_chirho {
            self.contact_phone_chirho = contact_phone_chirho;
        }
        if let Some(website_url_chirho) = update_chirho.website_url_chirho {
            self.website_url_chirho = Some(website_url_chirho);
        }
        if let Some(capacity_chirho) = update_chirho.capacity_chirho {
            self.capacity_chirho = capacity_chirho;
        }
        if let Some(current_children_count_chirho) = update_chirho.current_children_count_chirho {
            self.current_children_count_chirho = current_children_count_chirho;
        }
        if let Some(founded_date_chirho) = update_chirho.founded_date_chirho {
            self.founded_date_chirho = founded_date_chirho;
        }
        if let Some(registration_number_chirho) = update_chirho.registration_number_chirho {
            self.registration_number_chirho = registration_number_chirho;
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrphanageVerificationChirho {
    pub orphanage_id_chirho: String,
    pub verified_by_chirho: String,
    pub verification_date_chirho: DateTime<Utc>,
} 