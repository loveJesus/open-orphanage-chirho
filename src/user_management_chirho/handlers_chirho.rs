// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router,
    routing::post,
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use axum_cloudflare_adapter::wasm_compat;

use crate::{
    AxumStateChirho,
};
use super::{
    models_chirho::{
        StaffUserChirho,
        SponsorUserChirho,
        UserRoleChirho,
        LoginRequestChirho,
        LoginResponseChirho,
    },
    db_chirho::UserDbChirho,
    auth_chirho::AuthChirho,
};

#[wasm_compat]
pub async fn create_staff_user_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(user_chirho): Json<StaffUserChirho>,
) -> impl IntoResponse {
    let mut user_chirho = user_chirho;
    user_chirho.user_id_chirho = Uuid::new_v4().to_string();
    user_chirho.created_at_chirho = Utc::now();
    user_chirho.updated_at_chirho = Utc::now();

    let auth_chirho = AuthChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    let hashed_password_chirho = match auth_chirho.hash_password_chirho(&user_chirho.hashed_password_chirho) {
        Ok(hash_chirho) => hash_chirho,
        Err(error_chirho) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    };
    user_chirho.hashed_password_chirho = hashed_password_chirho;

    let db_chirho = UserDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.create_staff_user_chirho(user_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(user_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn create_sponsor_user_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(user_chirho): Json<SponsorUserChirho>,
) -> impl IntoResponse {
    let mut user_chirho = user_chirho;
    user_chirho.user_id_chirho = Uuid::new_v4().to_string();
    user_chirho.created_at_chirho = Utc::now();
    user_chirho.updated_at_chirho = Utc::now();

    let auth_chirho = AuthChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    let hashed_password_chirho = match auth_chirho.hash_password_chirho(&user_chirho.hashed_password_chirho) {
        Ok(hash_chirho) => hash_chirho,
        Err(error_chirho) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    };
    user_chirho.hashed_password_chirho = hashed_password_chirho;

    let db_chirho = UserDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    match db_chirho.create_sponsor_user_chirho(user_chirho.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(json!(user_chirho))).into_response(),
        Err(error_chirho) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }
}

#[wasm_compat]
pub async fn login_handler_chirho(
    State(state_chirho): State<AxumStateChirho>,
    Json(login_request_chirho): Json<LoginRequestChirho>,
) -> impl IntoResponse {
    let db_chirho = UserDbChirho::new((*state_chirho.env_wrapper_chirho.env).clone());
    let auth_chirho = AuthChirho::new((*state_chirho.env_wrapper_chirho.env).clone());

    // Try to find the user in staff users first
    match db_chirho.get_staff_user_by_email_chirho(&login_request_chirho.email_chirho).await {
        Ok(Some(staff_user_chirho)) => {
            if auth_chirho.verify_password_chirho(&login_request_chirho.password_chirho, &staff_user_chirho.hashed_password_chirho).unwrap_or(false) {
                let token_chirho = match auth_chirho.generate_token_chirho(&staff_user_chirho.user_id_chirho, &staff_user_chirho.email_chirho, staff_user_chirho.role_chirho.clone()) {
                    Ok(token_chirho) => token_chirho,
                    Err(error_chirho) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
                };
                let login_response_chirho = LoginResponseChirho {
                    user_id_chirho: staff_user_chirho.user_id_chirho,
                    role_chirho: staff_user_chirho.role_chirho,
                    token_chirho,
                };
                return (StatusCode::OK, Json(json!(login_response_chirho))).into_response();
            }
        },
        Ok(None) => {},
        Err(error_chirho) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }

    // If not found in staff users, try sponsor users
    match db_chirho.get_sponsor_user_by_email_chirho(&login_request_chirho.email_chirho).await {
        Ok(Some(sponsor_user_chirho)) => {
            if auth_chirho.verify_password_chirho(&login_request_chirho.password_chirho, &sponsor_user_chirho.hashed_password_chirho).unwrap_or(false) {
                let token_chirho = match auth_chirho.generate_token_chirho(&sponsor_user_chirho.user_id_chirho, &sponsor_user_chirho.email_chirho, UserRoleChirho::SponsorChirho) {
                    Ok(token_chirho) => token_chirho,
                    Err(error_chirho) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
                };
                let login_response_chirho = LoginResponseChirho {
                    user_id_chirho: sponsor_user_chirho.user_id_chirho,
                    role_chirho: UserRoleChirho::SponsorChirho,
                    token_chirho,
                };
                return (StatusCode::OK, Json(json!(login_response_chirho))).into_response();
            }
        },
        Ok(None) => {},
        Err(error_chirho) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error_chirho.to_string()}))).into_response(),
    }

    (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))).into_response()
}

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/api_chirho/v1_chirho/users_chirho/staff_chirho", post(create_staff_user_handler_chirho))
        .route("/api_chirho/v1_chirho/users_chirho/sponsor_chirho", post(create_sponsor_user_handler_chirho))
        .route("/api_chirho/v1_chirho/login_chirho", post(login_handler_chirho))
} 