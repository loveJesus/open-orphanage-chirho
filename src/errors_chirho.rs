// For God so loved the world, that he gave his only begotten Son,
// that all who believe in Him should not perish but have everlasting life.
// John 3:16 (KJV)

use worker::Response;
use serde_json::json;

#[derive(Debug)]
pub enum ErrorChirho {
    DatabaseErrorChirho(String),
    ValidationErrorChirho(String),
    NotFoundErrorChirho(String),
    AuthErrorChirho(String),
    UnauthorizedErrorChirho(String),
    BadRequestErrorChirho(String),
    InternalErrorChirho(String),
    SerializationErrorChirho(String),
    WorkerErrorChirho(String),
}

impl From<ErrorChirho> for worker::Error {
    fn from(err: ErrorChirho) -> Self {
        worker::Error::RustError(format!("{:?}", err))
    }
}

impl std::fmt::Display for ErrorChirho {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorChirho::DatabaseErrorChirho(msg) => write!(f, "Database error: {}", msg),
            ErrorChirho::ValidationErrorChirho(msg) => write!(f, "Validation error: {}", msg),
            ErrorChirho::NotFoundErrorChirho(msg) => write!(f, "Not found: {}", msg),
            ErrorChirho::AuthErrorChirho(msg) => write!(f, "Authentication error: {}", msg),
            ErrorChirho::UnauthorizedErrorChirho(msg) => write!(f, "Unauthorized: {}", msg),
            ErrorChirho::BadRequestErrorChirho(msg) => write!(f, "Bad request: {}", msg),
            ErrorChirho::InternalErrorChirho(msg) => write!(f, "Internal error: {}", msg),
            ErrorChirho::SerializationErrorChirho(msg) => write!(f, "Serialization error: {}", msg),
            ErrorChirho::WorkerErrorChirho(msg) => write!(f, "Worker error: {}", msg),
        }
    }
}

impl std::error::Error for ErrorChirho {}

impl From<worker::Error> for ErrorChirho {
    fn from(e: worker::Error) -> Self {
        ErrorChirho::WorkerErrorChirho(e.to_string())
    }
}

impl From<serde_json::Error> for ErrorChirho {
    fn from(e: serde_json::Error) -> Self {
        ErrorChirho::SerializationErrorChirho(e.to_string())
    }
}

impl From<ErrorChirho> for Response {
    fn from(error_chirho: ErrorChirho) -> Self {
        let (status_code_chirho, message_chirho) = match error_chirho {
            ErrorChirho::DatabaseErrorChirho(msg) => (500, msg),
            ErrorChirho::ValidationErrorChirho(msg) => (400, msg),
            ErrorChirho::NotFoundErrorChirho(msg) => (404, msg),
            ErrorChirho::AuthErrorChirho(msg) => (401, msg),
            ErrorChirho::UnauthorizedErrorChirho(msg) => (403, msg),
            ErrorChirho::BadRequestErrorChirho(msg) => (400, msg),
            ErrorChirho::InternalErrorChirho(msg) => (500, msg),
            ErrorChirho::SerializationErrorChirho(msg) => (500, msg),
            ErrorChirho::WorkerErrorChirho(msg) => (500, msg),
        };

        Response::from_json(&json!({
            "error": message_chirho
        }))
        .unwrap_or_else(|_| Response::error("Internal Server Error", 500).unwrap())
        .with_status(status_code_chirho)
    }
} 