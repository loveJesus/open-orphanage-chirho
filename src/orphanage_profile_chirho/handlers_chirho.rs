// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router,
    routing::{post, get, put, delete},
    extract::{State, Path, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use axum_cloudflare_adapter::wasm_compat;

use crate::AxumStateChirho;
use super::{
    models_chirho::{OrphanageProfileChirho, OrphanageUpdateChirho, OrphanageVerificationChirho},
    db_chirho::OrphanageDbChirho,
};

#[wasm_compat]
pub async fn create_orphanage_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(mut profile_chirho): Json<OrphanageProfileChirho>,
) -> impl IntoResponse {
    profile_chirho.orphanage_id_chirho = Uuid::new_v4().to_string();
    profile_chirho.created_at_chirho = Utc::now();
    profile_chirho.updated_at_chirho = Utc::now();

    let db_chirho = OrphanageDbChirho::new((*state_chirho.env_wrapper.env).d1("DB_CHIRHO").unwrap());
    match db_chirho.create_orphanage_profile_chirho(profile_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(profile_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_orphanage_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(orphanage_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = OrphanageDbChirho::new((*state_chirho.env_wrapper.env).d1("DB_CHIRHO").unwrap());
    match db_chirho.get_orphanage_profile_chirho(&orphanage_id_chirho).await {
        Ok(Some(profile_chirho)) => (StatusCode::OK, Json(json!(profile_chirho))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Orphanage profile not found"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn update_orphanage_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(orphanage_id_chirho): Path<String>,
    Json(update_chirho): Json<OrphanageUpdateChirho>,
) -> impl IntoResponse {
    let db_chirho = OrphanageDbChirho::new((*state_chirho.env_wrapper.env).d1("DB_CHIRHO").unwrap());
    // Fetch existing profile
    match db_chirho.get_orphanage_profile_chirho(&orphanage_id_chirho).await {
        Ok(Some(mut profile_chirho)) => {
            // Apply updates
            if let Some(name_chirho) = update_chirho.name_chirho { profile_chirho.name_chirho = name_chirho; }
            if let Some(description_chirho) = update_chirho.description_chirho { profile_chirho.description_chirho = description_chirho; }
            if let Some(address_chirho) = update_chirho.address_chirho { profile_chirho.address_chirho = address_chirho; }
            if let Some(contact_email_chirho) = update_chirho.contact_email_chirho { profile_chirho.contact_email_chirho = contact_email_chirho; }
            if let Some(contact_phone_chirho) = update_chirho.contact_phone_chirho { profile_chirho.contact_phone_chirho = contact_phone_chirho; }
            if let Some(website_url_chirho) = update_chirho.website_url_chirho { profile_chirho.website_url_chirho = Some(website_url_chirho); }
            if let Some(capacity_chirho) = update_chirho.capacity_chirho { profile_chirho.capacity_chirho = capacity_chirho; }
            if let Some(current_children_count_chirho) = update_chirho.current_children_count_chirho { profile_chirho.current_children_count_chirho = current_children_count_chirho; }
            if let Some(founded_date_chirho) = update_chirho.founded_date_chirho { profile_chirho.founded_date_chirho = founded_date_chirho; }
            if let Some(registration_number_chirho) = update_chirho.registration_number_chirho { profile_chirho.registration_number_chirho = registration_number_chirho; }
            profile_chirho.updated_at_chirho = Utc::now();
            // Save
            match db_chirho.update_orphanage_profile_chirho(profile_chirho.clone()).await {
                Ok(_) => (StatusCode::OK, Json(json!(profile_chirho))).into_response(),
                Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Orphanage profile not found"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn delete_orphanage_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(orphanage_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = OrphanageDbChirho::new((*state_chirho.env_wrapper.env).d1("DB_CHIRHO").unwrap());
    match db_chirho.delete_orphanage_profile_chirho(&orphanage_id_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Orphanage profile deleted successfully"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn verify_orphanage_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(orphanage_id_chirho): Path<String>,
    Json(mut verification_chirho): Json<OrphanageVerificationChirho>,
) -> impl IntoResponse {
    verification_chirho.orphanage_id_chirho = orphanage_id_chirho;
    verification_chirho.verification_date_chirho = Utc::now();

    let db_chirho = OrphanageDbChirho::new((*state_chirho.env_wrapper.env).d1("DB_CHIRHO").unwrap());
    match db_chirho.verify_orphanage_chirho(verification_chirho.clone()).await {
        Ok(_) => (StatusCode::OK, Json(json!(verification_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/orphanages_chirho", post(create_orphanage_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/orphanages_chirho/:orphanage_id_chirho", get(get_orphanage_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/orphanages_chirho/:orphanage_id_chirho", put(update_orphanage_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/orphanages_chirho/:orphanage_id_chirho", delete(delete_orphanage_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/orphanages_chirho/:orphanage_id_chirho/verify_chirho", post(verify_orphanage_handler_chirho))
} 