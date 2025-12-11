use actix_web::error::JsonPayloadError;
use sqlx::Error as SQLXError;

use crate::api::types::JsonError;

pub(super) fn map_input_err(err: JsonPayloadError) -> JsonError {
    match err {
        JsonPayloadError::Overflow { limit } => JsonError::PayloadTooLarge(format!(
            "You're payload is greater than the limit for {} bytes",
            limit
        )),
        JsonPayloadError::OverflowKnownLength { length, limit } => {
            JsonError::PayloadTooLarge(format!(
                "You're payload length of {} bytes is greater than the limit for {} bytes",
                length, limit
            ))
        }
        JsonPayloadError::ContentType => JsonError::UnsupportedMediaType(
            "Unsupported 'Content-Type' header or missing 'Content-Type' header".to_string(),
        ),
        JsonPayloadError::Payload(e) => {
            JsonError::BadRequest(format!("Error processing your payload: {}", e))
        }
        JsonPayloadError::Deserialize(e) => {
            JsonError::BadRequest(format!("Error deserializing your payload: {}", e))
        }
        _ => JsonError::Unknown(format!("Unknown JSON payload error: {}", err)),
    }
}

pub(super) fn map_query_err(err: SQLXError) -> JsonError {
    match err {
        SQLXError::InvalidArgument(msg) => {
            JsonError::BadRequest(format!("Invalid Argument Provided: {}", msg))
        }
        _ => JsonError::ServerError(format!("Database Insertion Error: {}", err)),
    }
}
