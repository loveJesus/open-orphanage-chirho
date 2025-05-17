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

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Request received: {}", req.url()?);
    
    let router = Router::new();
    
    router
        .get_async("/api_chirho/v1_chirho/health_chirho", |_, _| async move {
            Ok(Response::ok("OpenOrphanageChirho API is healthy")?)
        })
        .run(req, env)
        .await
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
