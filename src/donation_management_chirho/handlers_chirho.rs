// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router,
    routing::{get, post, put},
    extract::{State, Path, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    AxumStateChirho,
    errors_chirho::ErrorChirho,
};
use super::{
    models_chirho::{
        DonationChirho,
        DonationStatusChirho,
    },
    db_chirho::DonationDbChirho,
};

#[axum::debug_handler]
pub async fn create_donation_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(donation_chirho): Json<DonationChirho>,
) -> impl IntoResponse {
    let mut donation_chirho = donation_chirho;
    donation_chirho.donation_id_chirho = Uuid::new_v4().to_string();
    donation_chirho.status_chirho = DonationStatusChirho::PendingChirho;
    donation_chirho.created_at_chirho = Utc::now();
    donation_chirho.updated_at_chirho = Utc::now();

    let db_chirho = DonationDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.create_donation_chirho(donation_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(donation_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_donation_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(donation_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = DonationDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.get_donation_chirho(&donation_id_chirho).await {
        Ok(Some(donation_chirho)) => (StatusCode::OK, Json(json!(donation_chirho))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Donation not found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn update_donation_status_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(donation_id_chirho): Path<String>,
    Json(status_chirho): Json<DonationStatusChirho>,
) -> impl IntoResponse {
    let db_chirho = DonationDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.update_donation_status_chirho(&donation_id_chirho, status_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Donation status updated successfully"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_donor_donations_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(donor_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = DonationDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.get_donor_donations_chirho(&donor_id_chirho).await {
        Ok(donations_chirho) => (StatusCode::OK, Json(json!(donations_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_orphanage_donations_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(orphanage_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = DonationDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.get_orphanage_donations_chirho(&orphanage_id_chirho).await {
        Ok(donations_chirho) => (StatusCode::OK, Json(json!(donations_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/donations_chirho", post(create_donation_handler_chirho))
        .route("/api_chirho/v1_chirho/donations_chirho/:donation_id_chirho", get(get_donation_handler_chirho))
        .route("/api_chirho/v1_chirho/donations_chirho/:donation_id_chirho/status_chirho", put(update_donation_status_handler_chirho))
        .route("/api_chirho/v1_chirho/donors_chirho/:donor_id_chirho/donations_chirho", get(get_donor_donations_handler_chirho))
        .route("/api_chirho/v1_chirho/orphanages_chirho/:orphanage_id_chirho/donations_chirho", get(get_orphanage_donations_handler_chirho))
} 