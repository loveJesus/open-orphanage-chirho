// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{State, Path, Json},
    response::IntoResponse,
    http::StatusCode,
};
use axum_cloudflare_adapter::wasm_compat;
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    AxumStateChirho,
};
use super::{
    models_chirho::{
        SponsorshipChirho,
        SponsorshipStatusChirho,
        SponsorshipUpdateChirho,
    },
    db_chirho::SponsorshipDbChirho,
};

#[wasm_compat]
pub async fn create_sponsorship_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(sponsorship_chirho): Json<SponsorshipChirho>,
) -> impl IntoResponse {
    let mut sponsorship_chirho = sponsorship_chirho;
    sponsorship_chirho.sponsorship_id_chirho = Uuid::new_v4().to_string();
    sponsorship_chirho.status_chirho = SponsorshipStatusChirho::ActiveChirho;
    sponsorship_chirho.created_at_chirho = Utc::now();
    sponsorship_chirho.updated_at_chirho = Utc::now();

    let db_chirho = SponsorshipDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.create_sponsorship_chirho(sponsorship_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(sponsorship_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_sponsorship_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(sponsorship_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = SponsorshipDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.get_sponsorship_chirho(&sponsorship_id_chirho).await {
        Ok(Some(sponsorship_chirho)) => (StatusCode::OK, Json(json!(sponsorship_chirho))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Sponsorship not found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn update_sponsorship_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(sponsorship_id_chirho): Path<String>,
    Json(update_chirho): Json<SponsorshipUpdateChirho>,
) -> impl IntoResponse {
    let db_chirho = SponsorshipDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.update_sponsorship_chirho(&sponsorship_id_chirho, update_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Sponsorship updated successfully"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn delete_sponsorship_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(sponsorship_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = SponsorshipDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.delete_sponsorship_chirho(&sponsorship_id_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Sponsorship deleted successfully"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_sponsor_sponsorships_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(sponsor_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = SponsorshipDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.get_sponsor_sponsorships_chirho(&sponsor_id_chirho).await {
        Ok(sponsorships_chirho) => (StatusCode::OK, Json(json!(sponsorships_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_child_sponsorships_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(child_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = SponsorshipDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.get_child_sponsorships_chirho(&child_id_chirho).await {
        Ok(sponsorships_chirho) => (StatusCode::OK, Json(json!(sponsorships_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/sponsorships_chirho", post(create_sponsorship_handler_chirho))
        .route("/api_chirho/v1_chirho/sponsorships_chirho/:sponsorship_id_chirho", get(get_sponsorship_handler_chirho))
        .route("/api_chirho/v1_chirho/sponsorships_chirho/:sponsorship_id_chirho", put(update_sponsorship_handler_chirho))
        .route("/api_chirho/v1_chirho/sponsorships_chirho/:sponsorship_id_chirho", delete(delete_sponsorship_handler_chirho))
        .route("/api_chirho/v1_chirho/sponsors_chirho/:sponsor_id_chirho/sponsorships_chirho", get(get_sponsor_sponsorships_handler_chirho))
        .route("/api_chirho/v1_chirho/children_chirho/:child_id_chirho/sponsorships_chirho", get(get_child_sponsorships_handler_chirho))
} 