// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::*;
use uuid::Uuid;
use chrono::Utc;

use crate::errors_chirho::ErrorChirho;
use crate::utils_chirho::json_response_chirho;
use super::models_chirho::{OrphanageProfileChirho, OrphanageVerificationChirho};
use super::db_chirho::OrphanageDbChirho;

pub async fn create_orphanage_profile_handler_chirho(
    mut req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let mut orphanage_profile_chirho: OrphanageProfileChirho = req_chirho.json().await?;
    let now_chirho = Utc::now();
    orphanage_profile_chirho.created_at_chirho = now_chirho;
    orphanage_profile_chirho.updated_at_chirho = now_chirho;
    orphanage_profile_chirho.orphanage_id_chirho = Uuid::new_v4().to_string();

    let db_chirho = OrphanageDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    let created_orphanage_chirho = db_chirho.create_orphanage_profile_chirho(orphanage_profile_chirho).await?;

    Ok(Response::from_json(&created_orphanage_chirho)?)
}

pub async fn get_orphanage_profile_handler_chirho(
    _req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let orphanage_id_chirho = ctx_chirho.param("orphanage_id_chirho")
        .ok_or_else(|| ErrorChirho::BadRequestErrorChirho("Missing orphanage_id_chirho parameter".to_string()))?;

    let db_chirho = OrphanageDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    let orphanage_profile_chirho = db_chirho.get_orphanage_profile_chirho(orphanage_id_chirho).await?;

    match orphanage_profile_chirho {
        Some(profile_chirho) => Ok(Response::from_json(&profile_chirho)?),
        None => Err(ErrorChirho::NotFoundErrorChirho("Orphanage profile not found".to_string()).into()),
    }
}

pub async fn update_orphanage_profile_handler_chirho(
    mut req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let orphanage_id_chirho = ctx_chirho.param("orphanage_id_chirho")
        .ok_or_else(|| ErrorChirho::BadRequestErrorChirho("Missing orphanage_id_chirho parameter".to_string()))?;

    let mut orphanage_profile_chirho: OrphanageProfileChirho = req_chirho.json().await?;
    orphanage_profile_chirho.orphanage_id_chirho = orphanage_id_chirho.to_string();
    orphanage_profile_chirho.updated_at_chirho = Utc::now();

    let db_chirho = OrphanageDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    let updated_orphanage_chirho = db_chirho.update_orphanage_profile_chirho(orphanage_profile_chirho).await?;

    Ok(Response::from_json(&updated_orphanage_chirho)?)
}

pub async fn delete_orphanage_profile_handler_chirho(
    _req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let orphanage_id_chirho = ctx_chirho.param("orphanage_id_chirho")
        .ok_or_else(|| ErrorChirho::BadRequestErrorChirho("Missing orphanage_id_chirho parameter".to_string()))?;

    let db_chirho = OrphanageDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    db_chirho.delete_orphanage_profile_chirho(orphanage_id_chirho).await?;

    Response::ok("Orphanage profile deleted successfully")
}

pub async fn verify_orphanage_handler_chirho(
    mut req_chirho: Request,
    env_chirho: Env,
) -> Result<Response> {
    let url_chirho = req_chirho.url()?;
    let path_segments_chirho: Vec<&str> = url_chirho.path().split('/').collect();
    let orphanage_id_chirho = path_segments_chirho.last()
        .ok_or_else(|| ErrorChirho::ValidationErrorChirho("Orphanage ID not provided".to_string()))?;

    let mut verification_chirho: OrphanageVerificationChirho = req_chirho.json().await?;
    verification_chirho.orphanage_id_chirho = orphanage_id_chirho.to_string();
    verification_chirho.verification_date_chirho = Utc::now();

    let d1_db_chirho = env_chirho.d1("DB_CHIRHO")?;
    let db_chirho = OrphanageDbChirho::new(d1_db_chirho);
    db_chirho.verify_orphanage_chirho(verification_chirho.clone()).await?;

    Ok(json_response_chirho(&verification_chirho)?)
} 