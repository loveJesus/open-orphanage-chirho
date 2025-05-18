// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::{
    Router as AxumRouter,
    routing::get,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use axum::response::Html;
use serde_json::json;
use axum_cloudflare_adapter::{to_axum_request, to_worker_response, wasm_compat, EnvWrapper};
use worker::*;
use tower_service::Service;
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

use cfg_if::cfg_if;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook_chirho() {}
    }
}

#[derive(Clone)]
pub struct AxumStateChirho {
    pub env_wrapper_chirho: EnvWrapper,
}

#[wasm_compat]
pub async fn index_chirho(State(_state_chirho): State<AxumStateChirho>) -> impl IntoResponse {
    // (StatusCode::OK, Json(json!({
    //     "message": "Welcome to OpenOrphanageChirho API",
    //     "version": "1.0.0"
    // })))
    axum::response::Response::builder()
        .header("CONTENT-TYPE", "text/html")
        .body("Jesus Christ is Lord".to_owned())
        .unwrap()
}

pub fn get_routes_chirho() -> AxumRouter<AxumStateChirho> {
    AxumRouter::new()
        .route("/", get(index_chirho))
        .merge(get_user_routes_chirho())
        .merge(get_orphanage_routes_chirho())
        .merge(get_child_routes_chirho())
        .merge(get_sponsorship_routes_chirho())
        .merge(get_donation_routes_chirho())
        .merge(get_communication_routes_chirho())
        .merge(get_needs_routes_chirho())
}

fn log_request_chirho(request_chirho: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        request_chirho.path(),
        request_chirho.cf().unwrap().coordinates().unwrap_or_default(),
        request_chirho.cf().unwrap().region().unwrap_or_else(|| "unknown region".into())
    );
}
#[event(fetch)]
pub async fn main(request_chirho: Request, env_chirho: Env, _ctx_chirho: worker::Context) -> Result<Response> {
    log_request_chirho(&request_chirho);
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    set_panic_hook_chirho();

    let axum_state = AxumStateChirho {
        env_wrapper_chirho: EnvWrapper::new(env_chirho),
    };
    console_log!("JESUS IS LORD");

    let mut router_partial_chirho = get_routes_chirho();

    console_log!("JESUS IS KING");

    let mut router_chirho = router_partial_chirho
        .with_state(axum_state);

    console_log!("JESUS IS ALWAYS LORD");


    let axum_request = to_axum_request(request_chirho).await.unwrap();

    console_log!("JESUS IS LORD OF LORDS");

    let axum_response = router_chirho.as_service().call(axum_request).await.unwrap();
    console_log!("JESUS IS KING OF KINGS");

    let response = to_worker_response(axum_response).await.unwrap();

    console_log!("HALLELUJAH");


    Ok(response)
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
