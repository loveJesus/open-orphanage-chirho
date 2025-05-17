// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::*;
use uuid::Uuid;
use chrono::Utc;

use crate::errors_chirho::ErrorChirho;
use crate::utils_chirho::json_response_chirho;
use super::models_chirho::{ChildProfileChirho, ChildUpdateChirho};
use super::db_chirho::ChildDbChirho;

pub async fn create_child_profile_handler_chirho(
    mut req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let mut child_profile_chirho: ChildProfileChirho = req_chirho.json().await?;
    
    // Set timestamps and IDs
    let now_chirho = Utc::now();
    child_profile_chirho.created_at_chirho = now_chirho;
    child_profile_chirho.updated_at_chirho = now_chirho;
    child_profile_chirho.child_id_chirho = Uuid::new_v4().to_string();

    let db_chirho = ChildDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    let created_child_chirho = db_chirho.create_child_profile_chirho(child_profile_chirho).await?;

    Ok(Response::from_json(&created_child_chirho)?)
}

pub async fn get_child_profile_handler_chirho(
    _req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let child_id_chirho = ctx_chirho.param("child_id_chirho")
        .ok_or_else(|| ErrorChirho::BadRequestErrorChirho("Missing child_id_chirho parameter".to_string()))?;

    let db_chirho = ChildDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    let child_profile_chirho = db_chirho.get_child_profile_chirho(child_id_chirho).await?;

    match child_profile_chirho {
        Some(profile_chirho) => Ok(Response::from_json(&profile_chirho)?),
        None => Err(ErrorChirho::NotFoundErrorChirho("Child profile not found".to_string()).into()),
    }
}

pub async fn get_children_by_orphanage_handler_chirho(
    _req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let orphanage_id_chirho = ctx_chirho.param("orphanage_id_chirho")
        .ok_or_else(|| ErrorChirho::BadRequestErrorChirho("Missing orphanage_id_chirho parameter".to_string()))?;

    let db_chirho = ChildDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    let children_chirho = db_chirho.get_children_by_orphanage_chirho(orphanage_id_chirho).await?;

    Ok(Response::from_json(&children_chirho)?)
}

pub async fn update_child_profile_handler_chirho(
    mut req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let child_id_chirho = ctx_chirho.param("child_id_chirho")
        .ok_or_else(|| ErrorChirho::BadRequestErrorChirho("Missing child_id_chirho parameter".to_string()))?;

    let mut child_profile_chirho: ChildProfileChirho = req_chirho.json().await?;
    child_profile_chirho.child_id_chirho = child_id_chirho.to_string();
    child_profile_chirho.updated_at_chirho = Utc::now();

    let db_chirho = ChildDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    let updated_child_chirho = db_chirho.update_child_profile_chirho(child_profile_chirho).await?;

    Ok(Response::from_json(&updated_child_chirho)?)
}

pub async fn delete_child_profile_handler_chirho(
    _req_chirho: Request,
    ctx_chirho: RouteContext<()>,
) -> Result<Response> {
    let child_id_chirho = ctx_chirho.param("child_id_chirho")
        .ok_or_else(|| ErrorChirho::BadRequestErrorChirho("Missing child_id_chirho parameter".to_string()))?;

    let db_chirho = ChildDbChirho::new(ctx_chirho.env.d1("DB_CHIRHO")?);
    db_chirho.delete_child_profile_chirho(child_id_chirho).await?;

    Response::ok("Child profile deleted successfully")
}

pub async fn create_child_update_handler_chirho(
    mut req_chirho: Request,
    env_chirho: Env,
) -> Result<Response> {
    let url_chirho = req_chirho.url()?;
    let path_segments_chirho: Vec<&str> = url_chirho.path().split('/').collect();
    let child_id_chirho = path_segments_chirho.last()
        .ok_or_else(|| ErrorChirho::ValidationErrorChirho("Child ID not provided".to_string()))?;

    let mut update_chirho: ChildUpdateChirho = req_chirho.json().await?;
    update_chirho.update_id_chirho = Uuid::new_v4().to_string();
    update_chirho.child_id_chirho = child_id_chirho.to_string();
    update_chirho.date_posted_chirho = Utc::now();

    let db_chirho = ChildDbChirho::new(env_chirho.d1("DB_CHIRHO")?);
    db_chirho.create_child_update_chirho(update_chirho.clone()).await?;

    Ok(json_response_chirho(&update_chirho)?)
} 