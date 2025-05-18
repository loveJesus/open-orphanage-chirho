// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router,
    routing::{get, post},
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
    models_chirho::{MessageChirho, MessageStatusChirho},
    db_chirho::MessageDbChirho,
};

// HTTP handlers for communication_chirho module. 

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/messages_chirho", post(create_message_handler_chirho))
        .route("/api_chirho/v1_chirho/messages_chirho/{message_id_chirho}", get(get_message_handler_chirho))
        .route("/api_chirho/v1_chirho/messages_chirho/{message_id_chirho}/status_chirho", post(update_message_status_handler_chirho))
        .route("/api_chirho/v1_chirho/sponsorships_chirho/{sponsorship_id_chirho}/messages_chirho", get(get_messages_by_sponsorship_handler_chirho))
} 

#[wasm_compat]
pub async fn create_message_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(message_chirho): Json<MessageChirho>,
) -> impl IntoResponse {
    let mut message_chirho = message_chirho;
    message_chirho.message_id_chirho = Uuid::new_v4().to_string();
    message_chirho.moderation_status_chirho = MessageStatusChirho::PendingChirho;
    message_chirho.created_at_chirho = Utc::now();
    message_chirho.updated_at_chirho = Utc::now();

    let db_chirho = MessageDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.create_message_chirho(message_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(message_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_message_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(message_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = MessageDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.get_message_chirho(&message_id_chirho).await {
        Ok(Some(message_chirho)) => (StatusCode::OK, Json(json!(message_chirho))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Message not found"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn update_message_status_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(message_id_chirho): Path<String>,
    Json(status_chirho): Json<MessageStatusChirho>,
) -> impl IntoResponse {
    let db_chirho = MessageDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.update_message_status_chirho(&message_id_chirho, status_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Status updated successfully"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_messages_by_sponsorship_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(sponsorship_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = MessageDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.get_messages_by_sponsorship_chirho(&sponsorship_id_chirho).await {
        Ok(messages_chirho) => (StatusCode::OK, Json(json!(messages_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
} 