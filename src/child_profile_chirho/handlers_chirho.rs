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
use serde_json::json;
use axum_cloudflare_adapter::wasm_compat;

use crate::AxumStateChirho;
use super::models_chirho::{ChildProfileChirho, ChildUpdateChirho};
use super::db_chirho::ChildDbChirho;

#[wasm_compat]
pub async fn create_child_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(child_profile_chirho): Json<ChildProfileChirho>,
) -> impl IntoResponse {
    let db_chirho = ChildDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.create_child_profile_chirho(child_profile_chirho).await {
        Ok(profile_chirho) => (StatusCode::CREATED, Json(json!(profile_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_child_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(child_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = ChildDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.get_child_profile_chirho(&child_id_chirho).await {
        Ok(Some(profile_chirho)) => (StatusCode::OK, Json(json!(profile_chirho))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Child profile not found"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn get_orphanage_children_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(orphanage_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = ChildDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.get_children_by_orphanage_chirho(&orphanage_id_chirho).await {
        Ok(children_chirho) => (StatusCode::OK, Json(json!(children_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn update_child_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(child_id_chirho): Path<String>,
    Json(update_chirho): Json<ChildProfileChirho>,
) -> impl IntoResponse {
    let db_chirho = ChildDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    
    // First check if the child profile exists
    match db_chirho.get_child_profile_chirho(&child_id_chirho).await {
        Ok(Some(_)) => {
            // Update the profile
            match db_chirho.update_child_profile_chirho(update_chirho).await {
                Ok(profile_chirho) => (StatusCode::OK, Json(json!(profile_chirho))).into_response(),
                Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Child profile not found"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn delete_child_profile_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(child_id_chirho): Path<String>,
) -> impl IntoResponse {
    let db_chirho = ChildDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    
    // First check if the child profile exists
    match db_chirho.get_child_profile_chirho(&child_id_chirho).await {
        Ok(Some(_)) => {
            // Delete the profile
            match db_chirho.delete_child_profile_chirho(&child_id_chirho).await {
                Ok(_) => (StatusCode::NO_CONTENT, Json(json!({"message": "Child profile deleted successfully"}))).into_response(),
                Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Child profile not found"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn post_child_update_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Path(child_id_chirho): Path<String>,
    Json(update_chirho): Json<ChildUpdateChirho>,
) -> impl IntoResponse {
    let db_chirho = ChildDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    
    // First check if the child profile exists
    match db_chirho.get_child_profile_chirho(&child_id_chirho).await {
        Ok(Some(_)) => {
            // Create the update
            match db_chirho.create_child_update_chirho(update_chirho).await {
                Ok(_) => (StatusCode::CREATED, Json(json!({"message": "Child update posted successfully"}))).into_response(),
                Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Child profile not found"}))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/children_chirho", post(create_child_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/children_chirho/{child_id_chirho}", get(get_child_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/orphanages_chirho/{orphanage_id_chirho}/children_chirho", get(get_orphanage_children_handler_chirho))
        .route("/api_chirho/v1_chirho/children_chirho/{child_id_chirho}", put(update_child_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/children_chirho/{child_id_chirho}", delete(delete_child_profile_handler_chirho))
        .route("/api_chirho/v1_chirho/children_chirho/{child_id_chirho}/updates_chirho", post(post_child_update_handler_chirho))
} 