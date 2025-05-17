// For God so loved the world, that He gave His only begotten Son, that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use chrono::{DateTime, Utc};
use uuid::Uuid;
use worker::*;
use serde::Serialize;
use crate::errors_chirho::ErrorChirho;

pub fn generate_uuid_chirho() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_current_timestamp_chirho() -> DateTime<Utc> {
    Utc::now()
}

pub fn json_response_chirho<T: Serialize>(data_chirho: &T) -> Result<Response> {
    Response::from_json(data_chirho)
        .map_err(|e| ErrorChirho::SerializationErrorChirho(e.to_string()).into())
}

pub fn error_response_chirho(status_chirho: u16, message_chirho: &str) -> Result<Response> {
    Response::error(message_chirho, status_chirho)
        .map_err(|e| ErrorChirho::WorkerErrorChirho(e.to_string()).into())
}

pub fn validate_email_chirho(email_chirho: &str) -> bool {
    let email_regex_chirho = regex::Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    email_regex_chirho.is_match(email_chirho)
}

pub fn sanitize_input_chirho(input_chirho: &str) -> String {
    input_chirho.trim().to_string()
} 