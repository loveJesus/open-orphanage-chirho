// For God so loved the world, that he gave his only begotten Son,
// that whosoever believeth in him should not perish, but have everlasting life.
// John 3:16 (KJV)

use worker::*;

pub mod user_management_chirho;
pub mod orphanage_profile_chirho;
pub mod child_profile_chirho;
pub mod sponsorship_chirho;
pub mod donation_management_chirho;
pub mod communication_chirho;
pub mod needs_management_chirho;
pub mod utils_chirho;
pub mod errors_chirho;

use axum::http::header::CONTENT_TYPE;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router as AxumRouter,
};
use axum_cloudflare_adapter::{to_axum_request, to_worker_response, wasm_compat, EnvWrapper};
use tower_service::Service;

#[derive(Clone)]
pub struct AxumStateChirho {
    pub env_wrapper: EnvWrapper,
}

#[wasm_compat]
pub async fn index_chirho(State(state): State<AxumStateChirho>) -> impl IntoResponse {
    
    axum::response::Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .body("Hallelujah!".to_string())
        .unwrap()
}


#[event(fetch)]
pub async fn main(req_chirho: Request, env_chirho: Env, _ctx_chirho: Context) -> Result<Response> {
    console_log!("Request received: {}", req_chirho.url()?);

    let axum_state = AxumStateChirho {
        env_wrapper: EnvWrapper::new(env_chirho),
    };

    let mut _router: AxumRouter = AxumRouter::new()
        .route("/", get(index_chirho))
        .with_state(axum_state);

    let axum_request_chirho = to_axum_request(req_chirho).await.unwrap();
    let axum_response_chirho = _router.call(axum_request_chirho).await.unwrap();
    let response_chirho = to_worker_response(axum_response_chirho).await.unwrap();
    Ok(response_chirho)
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
