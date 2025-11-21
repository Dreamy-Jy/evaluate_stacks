use std::collections::HashSet;

use actix_web::{
    delete,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};

use crate::{
    api::{
        types::{JsonError, MaybeJson},
        utils::query_some,
    },
    db::sqlx::{
        delete_lists as db_delete_lists, delete_sets as db_delete_sets,
        delete_todos as db_delete_todos,
    },
    types::{ListID, SetID, SetQueryTarget, ToDoID, ToDoQueryTarget},
};

pub type DeleteListsRequest = HashSet<ListID>;
pub type DeleteSetsRequest = HashSet<SetQueryTarget>;
pub type DeleteToDosRequest = HashSet<ToDoQueryTarget>;

pub type DeleteListsResponse = HashSet<ListID>;
pub type DeleteSetsResponse = HashSet<SetID>;
pub type DeleteToDosResponse = HashSet<ToDoID>;

#[delete("/api/lists")]
pub async fn delete_lists(
    req: MaybeJson<DeleteListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<DeleteListsResponse>, JsonError> {
    query_some(req, db_conn_pool, db_delete_lists).await
}

#[delete("/api/sets")]
pub async fn delete_sets(
    req: MaybeJson<DeleteSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<DeleteSetsResponse>, JsonError> {
    query_some(req, db_conn_pool, db_delete_sets).await
}

#[delete("/api/to_dos")]
pub async fn delete_to_dos(
    req: MaybeJson<DeleteToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<DeleteToDosResponse>, JsonError> {
    query_some(req, db_conn_pool, db_delete_todos).await
}
