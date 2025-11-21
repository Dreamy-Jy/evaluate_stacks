use std::collections::HashSet;

use actix_web::{
    error::JsonPayloadError,
    post,
    web::{Data, Json},
};
use sqlx::{Error::InvalidArgument, Pool, Sqlite};

use crate::{
    api::types::{CreateList, CreateSet, CreateToDo, JsonError, MaybeJson},
    db::sqlx::{insert_lists, insert_sets, insert_todos},
    types::{List, Set, ToDo},
};

type CreateListsRequest = Vec<CreateList>;
type CreateSetsRequest = Vec<CreateSet>;
type CreateToDosRequest = Vec<CreateToDo>;

type CreateListsResponse = HashSet<List>;
type CreateSetsResponse = HashSet<Set>;
type CreateToDosResponse = HashSet<ToDo>;

#[post("/api/lists")]
pub async fn create_lists(
    req: MaybeJson<CreateListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<CreateListsResponse>, JsonError> {
    let lists;

    match req {
        MaybeJson::Valid(req) if req.len() == 0 || req.is_empty() => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Valid(req) => {
            lists = match insert_lists(db_conn_pool, req).await {
                Ok(lists) => lists,
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
            };
        }
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

#[post("/api/sets")]
pub async fn create_sets(
    req: MaybeJson<CreateSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<CreateSetsResponse>, JsonError> {
    let sets;

    match req {
        MaybeJson::Valid(req) if req.len() == 0 || req.is_empty() => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Valid(req) => {
            sets = match insert_sets(db_conn_pool, req).await {
                Ok(lists) => lists,
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
            };
        }
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

#[post("/api/to_dos")]
pub async fn create_to_dos(
    req: MaybeJson<CreateToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<CreateToDosResponse>, JsonError> {
    let todos;

    match req {
        MaybeJson::Valid(req) if req.len() == 0 || req.is_empty() => {
            return Err(JsonError::BadRequest(
                "Empty request not allowed".to_string(),
            ));
        }
        MaybeJson::Valid(req) => {
            todos = match insert_todos(db_conn_pool, req).await {
                Ok(lists) => lists,
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
            };
        }
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
