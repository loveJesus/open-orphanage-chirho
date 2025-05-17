// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router,
    routing::get,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde_json::json;
use axum_cloudflare_adapter::{wasm_compat, EnvWrapper};

use user_management_chirho::handlers_chirho::get_routes_chirho as get_user_routes_chirho;
use orphanage_profile_chirho::handlers_chirho::get_routes_chirho as get_orphanage_routes_chirho;
use child_profile_chirho::handlers_chirho::get_routes_chirho as get_child_routes_chirho;
use sponsorship_chirho::handlers_chirho::get_routes_chirho as get_sponsorship_routes_chirho;
use donation_management_chirho::handlers_chirho::get_routes_chirho as get_donation_routes_chirho;
use communication_chirho::handlers_chirho::get_routes_chirho as get_communication_routes_chirho;
use needs_management_chirho::handlers_chirho::get_routes_chirho as get_needs_routes_chirho;

pub mod user_management_chirho;
pub mod orphanage_profile_chirho;
pub mod child_profile_chirho;
pub mod sponsorship_chirho;
pub mod donation_management_chirho;
pub mod communication_chirho;
pub mod needs_management_chirho;
pub mod errors_chirho;
pub mod utils_chirho;

#[derive(Clone)]
pub struct AxumStateChirho {
    pub env_wrapper: EnvWrapper,
}

#[wasm_compat]
pub async fn index_chirho(State(_state): State<AxumStateChirho>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "message": "Welcome to OpenOrphanageChirho API",
        "version": "1.0.0"
    })))
}

pub fn get_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .route("/", get(index_chirho))
        .merge(get_user_routes_chirho())
        .merge(get_orphanage_routes_chirho())
        .merge(get_child_routes_chirho())
        .merge(get_sponsorship_routes_chirho())
        .merge(get_donation_routes_chirho())
        .merge(get_communication_routes_chirho())
        .merge(get_needs_routes_chirho())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
