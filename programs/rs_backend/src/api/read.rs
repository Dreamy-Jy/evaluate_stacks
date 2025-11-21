use crate::{
    api::types::{JsonError, MaybeJson},
    db::sqlx::{
        query_all_lists, query_all_sets, query_all_todos, query_lists, query_sets, query_todos,
    },
    types::{List, ListID, Set, SetQueryTarget, ToDo, ToDoQueryTarget},
};
use actix_web::{
    error::JsonPayloadError,
    get,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};
use std::collections::HashSet;

type ReadListsRequest = HashSet<ListID>;
type ReadSetsRequest = HashSet<SetQueryTarget>;
type ReadToDosRequest = HashSet<ToDoQueryTarget>;

type ReadListsResponse = HashSet<List>;
type ReadSetsResponse = HashSet<Set>;
type ReadToDosResponse = HashSet<ToDo>;

#[get("/api/lists")]
pub async fn read_lists(
    req: MaybeJson<ReadListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<ReadListsResponse>, JsonError> {
    let lists: HashSet<List>;

    match req {
        MaybeJson::Empty => {
            lists = query_all_lists(db_conn_pool).await.unwrap();
        }
        MaybeJson::Valid(json) if json.len() == 0 || json.is_empty() => {
            lists = query_all_lists(db_conn_pool).await.unwrap();
        }
        MaybeJson::Valid(json) => {
            lists = query_lists(db_conn_pool, json).await.unwrap();
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

#[get("/api/sets")]
pub async fn read_sets(
    req: MaybeJson<ReadSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<ReadSetsResponse>, JsonError> {
    let sets: HashSet<Set>;

    match req {
        MaybeJson::Empty => {
            sets = query_all_sets(db_conn_pool).await.unwrap();
        }
        MaybeJson::Valid(json) if json.len() == 0 || json.is_empty() => {
            sets = query_all_sets(db_conn_pool).await.unwrap();
        }
        MaybeJson::Valid(json) => {
            sets = query_sets(db_conn_pool, json).await.unwrap();
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

#[get("/api/to_dos")]
pub async fn read_to_dos(
    req: MaybeJson<ReadToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<ReadToDosResponse>, JsonError> {
    let todos: HashSet<ToDo>;

    match req {
        MaybeJson::Empty => {
            todos = query_all_todos(db_conn_pool).await.unwrap();
        }
        MaybeJson::Valid(json) if json.len() == 0 || json.is_empty() => {
            todos = query_all_todos(db_conn_pool).await.unwrap();
        }
        MaybeJson::Valid(json) => {
            todos = query_todos(db_conn_pool, json).await.unwrap();
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
