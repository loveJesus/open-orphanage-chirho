// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use axum::Router;
use crate::AxumStateChirho;

use crate::user_management_chirho::handlers_chirho::get_routes_chirho as get_user_routes_chirho;
use crate::orphanage_profile_chirho::handlers_chirho::get_routes_chirho as get_orphanage_routes_chirho;
use crate::communication_chirho::handlers_chirho::get_routes_chirho as get_communication_routes_chirho;
/*use crate::needs_management_chirho::handlers_chirho::get_routes_chirho as get_needs_routes_chirho;
use crate::donation_management_chirho::handlers_chirho::get_routes_chirho as get_donation_routes_chirho;
use crate::sponsorship_chirho::handlers_chirho::get_routes_chirho as get_sponsorship_routes_chirho;*/

pub fn get_all_routes_chirho() -> Router<AxumStateChirho> {
    Router::new()
        .merge(get_user_routes_chirho())
        .merge(get_orphanage_routes_chirho())
        .merge(get_communication_routes_chirho())
        /*.merge(get_needs_routes_chirho())
        .merge(get_donation_routes_chirho())
        .merge(get_sponsorship_routes_chirho())*/
} 