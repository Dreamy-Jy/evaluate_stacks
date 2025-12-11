use actix_web::{error::JsonPayloadError, web::Json};
use sqlx::Error as SQLXError;

use crate::api::types::JsonError;

pub(super) fn map_input_err<Out>(err: JsonPayloadError) -> Result<Json<Out>, JsonError> {
    match err {
        JsonPayloadError::Overflow { limit } => Err(JsonError::PayloadTooLarge(format!(
            "You're payload is greater than the limit for {} bytes",
            limit
        ))),
        JsonPayloadError::OverflowKnownLength { length, limit } => {
            Err(JsonError::PayloadTooLarge(format!(
                "You're payload length of {} bytes is greater than the limit for {} bytes",
                length, limit
            )))
        }
        JsonPayloadError::ContentType => Err(JsonError::UnsupportedMediaType(
            "Unsupported 'Content-Type' header or missing 'Content-Type' header".to_string(),
        )),
        JsonPayloadError::Payload(e) => Err(JsonError::BadRequest(format!(
            "Error processing your payload: {}",
            e
        ))),
        JsonPayloadError::Deserialize(e) => Err(JsonError::BadRequest(format!(
            "Error deserializing your payload: {}",
            e
        ))),
        _ => Err(JsonError::Unknown(format!(
            "Unknown JSON payload error: {}",
            err
        ))),
    }
}

pub(super) fn map_query_err<Out>(err: SQLXError) -> Result<Json<Out>, JsonError> {
    match err {
        SQLXError::InvalidArgument(msg) => Err(JsonError::BadRequest(format!(
            "Invalid Argument Provided: {}",
            msg
        ))),
        _ => Err(JsonError::ServerError(format!(
            "Database Insertion Error: {}",
            err
        ))),
    }
}
