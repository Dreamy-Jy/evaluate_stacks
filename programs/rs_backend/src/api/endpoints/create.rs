use std::collections::HashSet;

use actix_web::{
    post,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};

use crate::{
    api::{
        types::{JsonError, MaybeJson},
        utils::query_some,
    },
    db::sqlx::{insert_lists, insert_sets, insert_todos},
    types::{CreateList, CreateSet, CreateToDo, List, Set, ToDo},
};

pub type CreateListsRequest = Vec<CreateList>;
pub type CreateSetsRequest = Vec<CreateSet>;
pub type CreateToDosRequest = Vec<CreateToDo>;

pub type CreateListsResponse = HashSet<List>;
pub type CreateSetsResponse = HashSet<Set>;
pub type CreateToDosResponse = HashSet<ToDo>;

#[post("/api/lists")]
pub async fn create_lists(
    req: MaybeJson<CreateListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<CreateListsResponse>, JsonError> {
    query_some(req, db_conn_pool, insert_lists).await
}

#[post("/api/sets")]
pub async fn create_sets(
    req: MaybeJson<CreateSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<CreateSetsResponse>, JsonError> {
    query_some(req, db_conn_pool, insert_sets).await
}

#[post("/api/to_dos")]
pub async fn create_to_dos(
    req: MaybeJson<CreateToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<CreateToDosResponse>, JsonError> {
    query_some(req, db_conn_pool, insert_todos).await
}
