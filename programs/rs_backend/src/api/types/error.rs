use std::fmt::{Debug, Display};

use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};
use serde_json::json;

#[derive(Debug)]
pub enum JsonError {
    PayloadTooLarge(String),
    UnsupportedMediaType(String),
    BadRequest(String),
    ServerError(String),
    Unknown(String),
}

impl ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            JsonError::PayloadTooLarge(msg) => {
                HttpResponse::build(self.status_code()).json(json!({
                    "error": format!("Payload Too Large: {}", msg),
                }))
            }
            JsonError::UnsupportedMediaType(msg) => {
                HttpResponse::build(self.status_code()).json(json!({
                    "error": format!("Unsupported Media Type: {}", msg),
                }))
            }
            JsonError::ServerError(msg) => HttpResponse::build(self.status_code()).json(json!({
                "error": format!("Internal Server Error: {}", msg),
            })),
            JsonError::Unknown(msg) => HttpResponse::build(self.status_code()).json(json!({
                "error": format!("Unknown Error: {}", msg),
            })),
            JsonError::BadRequest(msg) => HttpResponse::build(self.status_code()).json(json!({
                "error": format!("Bad Request: {}", msg),
            })),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            JsonError::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            JsonError::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            JsonError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            JsonError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            JsonError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Json Error")
    }
}
