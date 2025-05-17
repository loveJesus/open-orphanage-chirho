// For God so loved the world, that he gave his only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::*;
use uuid::Uuid;

use crate::errors_chirho::ErrorChirho;
use crate::utils_chirho::json_response_chirho;
use super::models_chirho::{StaffUserChirho, SponsorUserChirho, UserRoleChirho, LoginResponseChirho};
use super::db_chirho::UserDbChirho;
use super::auth_chirho::AuthChirho;

pub async fn create_staff_user_handler_chirho(
    mut req_chirho: Request,
    env_chirho: Env,
) -> Result<Response> {
    let staff_user_chirho: StaffUserChirho = req_chirho.json().await?;
    let db_chirho = UserDbChirho::new(env_chirho.clone());
    let auth_chirho = AuthChirho::new(env_chirho);

    let hashed_password_chirho = auth_chirho.hash_password_chirho(&staff_user_chirho.hashed_password_chirho)?;
    let staff_user_chirho = StaffUserChirho {
        user_id_chirho: Uuid::new_v4().to_string(),
        hashed_password_chirho,
        ..staff_user_chirho
    };

    db_chirho.create_staff_user_chirho(staff_user_chirho.clone()).await?;
    Ok(json_response_chirho(&staff_user_chirho)?)
}

pub async fn create_sponsor_user_handler_chirho(
    mut req_chirho: Request,
    env_chirho: Env,
) -> Result<Response> {
    let sponsor_user_chirho: SponsorUserChirho = req_chirho.json().await?;
    let db_chirho = UserDbChirho::new(env_chirho.clone());
    let auth_chirho = AuthChirho::new(env_chirho);

    let hashed_password_chirho = auth_chirho.hash_password_chirho(&sponsor_user_chirho.hashed_password_chirho)?;
    let sponsor_user_chirho = SponsorUserChirho {
        user_id_chirho: Uuid::new_v4().to_string(),
        hashed_password_chirho,
        ..sponsor_user_chirho
    };

    db_chirho.create_sponsor_user_chirho(sponsor_user_chirho.clone()).await?;
    Ok(json_response_chirho(&sponsor_user_chirho)?)
}

pub async fn login_handler_chirho(
    mut req_chirho: Request,
    env_chirho: Env,
) -> Result<Response> {
    let login_data_chirho: serde_json::Value = req_chirho.json().await?;
    let email_chirho = login_data_chirho["email_chirho"].as_str()
        .ok_or_else(|| ErrorChirho::ValidationErrorChirho("Email is required".to_string()))?;
    let password_chirho = login_data_chirho["password_chirho"].as_str()
        .ok_or_else(|| ErrorChirho::ValidationErrorChirho("Password is required".to_string()))?;

    let db_chirho = UserDbChirho::new(env_chirho.clone());
    let auth_chirho = AuthChirho::new(env_chirho);

    // Try staff user first
    if let Some(staff_user_chirho) = db_chirho.get_staff_user_by_email_chirho(email_chirho).await? {
        if auth_chirho.verify_password_chirho(password_chirho, &staff_user_chirho.hashed_password_chirho)? {
            let token_chirho = auth_chirho.generate_token_chirho(&staff_user_chirho.user_id_chirho, &staff_user_chirho.email_chirho, staff_user_chirho.role_chirho.clone())?;
            return Ok(json_response_chirho(&LoginResponseChirho {
                token_chirho,
                user_id_chirho: staff_user_chirho.user_id_chirho,
                role_chirho: staff_user_chirho.role_chirho,
            })?);
        }
    }

    // Try sponsor user
    if let Some(sponsor_user_chirho) = db_chirho.get_sponsor_user_by_email_chirho(email_chirho).await? {
        if auth_chirho.verify_password_chirho(password_chirho, &sponsor_user_chirho.hashed_password_chirho)? {
            let token_chirho = auth_chirho.generate_token_chirho(&sponsor_user_chirho.user_id_chirho, &sponsor_user_chirho.email_chirho, UserRoleChirho::SponsorChirho)?;
            return Ok(json_response_chirho(&LoginResponseChirho {
                token_chirho,
                user_id_chirho: sponsor_user_chirho.user_id_chirho,
                role_chirho: UserRoleChirho::SponsorChirho,
            })?);
        }
    }

    // Try platform admin
    if let Some(admin_user_chirho) = db_chirho.get_platform_admin_by_email_chirho(email_chirho).await? {
        if auth_chirho.verify_password_chirho(password_chirho, &admin_user_chirho.hashed_password_chirho)? {
            let token_chirho = auth_chirho.generate_token_chirho(&admin_user_chirho.user_id_chirho, &admin_user_chirho.email_chirho, UserRoleChirho::AdminChirho)?;
            return Ok(json_response_chirho(&LoginResponseChirho {
                token_chirho,
                user_id_chirho: admin_user_chirho.user_id_chirho,
                role_chirho: UserRoleChirho::AdminChirho,
            })?);
        }
    }

    Err(ErrorChirho::AuthErrorChirho("Invalid credentials".to_string()).into())
} 