// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
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

use crate::AxumStateChirho;
use super::{
    models_chirho::{
        NeedChirho,
        NeedStatusChirho,
        NeedUpdateChirho,
    },
    db_chirho::NeedDbChirho,
};

// HTTP handlers for needs_management_chirho module. 

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/needs_chirho", post(create_need_handler_chirho))
        .route("/api_chirho/v1_chirho/needs_chirho/:need_id_chirho", get(get_need_handler_chirho))
        .route("/api_chirho/v1_chirho/needs_chirho/:need_id_chirho", put(update_need_handler_chirho))
        .route("/api_chirho/v1_chirho/needs_chirho/:need_id_chirho", delete(delete_need_handler_chirho))
        .route("/api_chirho/v1_chirho/orphanages_chirho/:orphanage_id_chirho/needs_chirho", get(get_orphanage_needs_handler_chirho))
} 

#[wasm_compat]
pub async fn create_need_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(need_chirho): Json<NeedChirho>,
) -> impl IntoResponse {
    let mut need_chirho = need_chirho;
    need_chirho.need_id_chirho = Uuid::new_v4().to_string();
    need_chirho.status_chirho = NeedStatusChirho::ActiveChirho;
    need_chirho.created_at_chirho = Utc::now();
    need_chirho.updated_at_chirho = Utc::now();

    let db_chirho = NeedDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.create_need_chirho(need_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(need_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_need_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(need_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = NeedDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.get_need_chirho(&need_id_chirho).await {
        Ok(Some(need_chirho)) => (StatusCode::OK, Json(json!(need_chirho))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Need not found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn update_need_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(need_id_chirho): Path<String>,
    Json(need_update_chirho): Json<NeedUpdateChirho>,
) -> impl IntoResponse {
    let db_chirho = NeedDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.update_need_chirho(&need_id_chirho, need_update_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Need updated successfully"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn delete_need_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(need_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = NeedDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.delete_need_chirho(&need_id_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Need deleted successfully"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_orphanage_needs_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(orphanage_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = NeedDbChirho::new((*state_chirho.env_wrapper.env).clone());
    match db_chirho.get_orphanage_needs_chirho(&orphanage_id_chirho).await {
        Ok(needs_chirho) => (StatusCode::OK, Json(json!(needs_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
} 