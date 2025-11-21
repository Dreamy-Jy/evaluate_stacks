use std::collections::HashSet;

use actix_web::{
    delete,
    error::JsonPayloadError,
    web::{Data, Json},
};
use sqlx::{Error::InvalidArgument, Pool, Sqlite};

use crate::{
    api::types::{JsonError, MaybeJson},
    db::sqlx::{
        delete_lists as db_delete_lists, delete_sets as db_delete_sets,
        delete_todos as db_delete_todos,
    },
    types::{ListID, SetID, SetQueryTarget, ToDoID, ToDoQueryTarget},
};

type DeleteListsRequest = HashSet<ListID>;
type DeleteSetsRequest = HashSet<SetQueryTarget>;
type DeleteToDosRequest = HashSet<ToDoQueryTarget>;

type DeleteListsResponse = HashSet<ListID>;
type DeleteSetsResponse = HashSet<SetID>;
type DeleteToDosResponse = HashSet<ToDoID>;

#[delete("/api/lists")]
pub async fn delete_lists(
    req: MaybeJson<DeleteListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<DeleteListsResponse>, JsonError> {
    let lists;

    match req {
        MaybeJson::Valid(req) if req.len() == 0 || req.is_empty() => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Valid(req) => match db_delete_lists(db_conn_pool, req).await {
            Ok(ls) => {
                lists = ls;
            }
            Err(e) => match e {
                InvalidArgument(e) => {
                    return Err(JsonError::BadRequest(format!(
                        "Invalid Argument Provided: {}",
                        e
                    )));
                }
                _ => {
                    return Err(JsonError::ServerError(format!(
                        "Database Insertion Error: {}",
                        e
                    )));
                }
            },
        },
        MaybeJson::Empty => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Invalid(e) => match e {
            JsonPayloadError::Overflow { limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload is greater than the limit for {} bytes",
                    limit
                )));
            }
            JsonPayloadError::OverflowKnownLength { length, limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload length of {} bytes is greater than the limit for {} bytes",
                    length, limit
                )));
            }
            JsonPayloadError::ContentType => {
                return Err(JsonError::UnsupportedMediaType(
                    "Unsupported 'Content-Type' header or missing 'Content-Type' header"
                        .to_string(),
                ));
            }
            JsonPayloadError::Payload(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error processing your payload: {}",
                    e
                )));
            }
            JsonPayloadError::Deserialize(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error deserializing your payload: {}",
                    e
                )));
            }
            _ => {
                return Err(JsonError::BadRequest(format!(
                    "Unknown JSON payload error: {}",
                    e
                )));
            }
        },
    }

    Ok(Json(lists))
}

#[delete("/api/sets")]
pub async fn delete_sets(
    req: MaybeJson<DeleteSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<DeleteSetsResponse>, JsonError> {
    let sets;

    match req {
        MaybeJson::Valid(req) if req.len() == 0 || req.is_empty() => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Valid(req) => match db_delete_sets(db_conn_pool, req).await {
            Ok(ss) => {
                sets = ss;
            }
            Err(e) => match e {
                InvalidArgument(e) => {
                    return Err(JsonError::BadRequest(format!(
                        "Invalid Argument Provided: {}",
                        e
                    )));
                }
                _ => {
                    return Err(JsonError::ServerError(format!(
                        "Database Insertion Error: {}",
                        e
                    )));
                }
            },
        },
        MaybeJson::Empty => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Invalid(e) => match e {
            JsonPayloadError::Overflow { limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload is greater than the limit for {} bytes",
                    limit
                )));
            }
            JsonPayloadError::OverflowKnownLength { length, limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload length of {} bytes is greater than the limit for {} bytes",
                    length, limit
                )));
            }
            JsonPayloadError::ContentType => {
                return Err(JsonError::UnsupportedMediaType(
                    "Unsupported 'Content-Type' header or missing 'Content-Type' header"
                        .to_string(),
                ));
            }
            JsonPayloadError::Payload(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error processing your payload: {}",
                    e
                )));
            }
            JsonPayloadError::Deserialize(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error deserializing your payload: {}",
                    e
                )));
            }
            _ => {
                return Err(JsonError::BadRequest(format!(
                    "Unknown JSON payload error: {}",
                    e
                )));
            }
        },
    }

    Ok(Json(sets))
}

#[delete("/api/to_dos")]
pub async fn delete_to_dos(
    req: MaybeJson<DeleteToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<DeleteToDosResponse>, JsonError> {
    let todos;

    match req {
        MaybeJson::Valid(req) if req.len() == 0 || req.is_empty() => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Valid(req) => match db_delete_todos(db_conn_pool, req).await {
            Ok(tds) => {
                todos = tds;
            }
            Err(e) => match e {
                InvalidArgument(e) => {
                    return Err(JsonError::BadRequest(format!(
                        "Invalid Argument Provided: {}",
                        e
                    )));
                }
                _ => {
                    return Err(JsonError::ServerError(format!(
                        "Database Insertion Error: {}",
                        e
                    )));
                }
            },
        },
        MaybeJson::Empty => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Invalid(e) => match e {
            JsonPayloadError::Overflow { limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload is greater than the limit for {} bytes",
                    limit
                )));
            }
            JsonPayloadError::OverflowKnownLength { length, limit } => {
                return Err(JsonError::PayloadTooLarge(format!(
                    "You're payload length of {} bytes is greater than the limit for {} bytes",
                    length, limit
                )));
            }
            JsonPayloadError::ContentType => {
                return Err(JsonError::UnsupportedMediaType(
                    "Unsupported 'Content-Type' header or missing 'Content-Type' header"
                        .to_string(),
                ));
            }
            JsonPayloadError::Payload(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error processing your payload: {}",
                    e
                )));
            }
            JsonPayloadError::Deserialize(e) => {
                return Err(JsonError::BadRequest(format!(
                    "Error deserializing your payload: {}",
                    e
                )));
            }
            _ => {
                return Err(JsonError::BadRequest(format!(
                    "Unknown JSON payload error: {}",
                    e
                )));
            }
        },
    }

    Ok(Json(todos))
}
