// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router,
    routing::{get, post, put},
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
    errors_chirho::ErrorChirho,
};
use super::{
    models_chirho::{
        MessageChirho,
        MessageStatusChirho,
    },
};

// HTTP handlers for communication_chirho module. 

#[wasm_compat]
pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/messages_chirho", post(create_message_handler_chirho))
        .route("/api_chirho/v1_chirho/messages_chirho/:message_id_chirho", get(get_message_handler_chirho))
        .route("/api_chirho/v1_chirho/messages_chirho/:message_id_chirho/status_chirho", put(update_message_status_handler_chirho))
        .route("/api_chirho/v1_chirho/sponsorships_chirho/:sponsorship_id_chirho/messages_chirho", get(get_messages_by_sponsorship_handler_chirho))
} 

#[axum::debug_handler]
pub async fn create_message_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(message_chirho): Json<MessageChirho>,
) -> impl IntoResponse {
    let mut message_chirho = message_chirho;
    message_chirho.message_id_chirho = Uuid::new_v4().to_string();
    message_chirho.status_chirho = MessageStatusChirho::UnreadChirho;
    message_chirho.created_at_chirho = Utc::now();
    message_chirho.updated_at_chirho = Utc::now();

    let db_chirho = MessageDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.create_message_chirho(message_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(message_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_message_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(message_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = MessageDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.get_message_chirho(&message_id_chirho).await {
        Ok(Some(message_chirho)) => (StatusCode::OK, Json(json!(message_chirho))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Message not found"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn update_message_status_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(message_id_chirho): Path<String>,
    Json(status_chirho): Json<MessageStatusChirho>,
) -> impl IntoResponse {
    let db_chirho = MessageDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.update_message_status_chirho(&message_id_chirho, status_chirho).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Message status updated successfully"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_messages_by_sponsorship_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(sponsorship_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = MessageDbChirho::new(state_chirho.env_wrapper.env.d1("DB_CHIRHO").unwrap());
    match db_chirho.get_messages_by_sponsorship_chirho(&sponsorship_id_chirho).await {
        Ok(messages_chirho) => (StatusCode::OK, Json(json!(messages_chirho))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response(),
    }
} 