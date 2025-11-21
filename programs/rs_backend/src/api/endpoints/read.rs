use crate::{
    api::{
        types::{JsonError, MaybeJson},
        utils::query_all_or_some,
    },
    db::sqlx::{
        query_all_lists, query_all_sets, query_all_todos, query_lists, query_sets, query_todos,
    },
    types::{List, ListID, Set, SetQueryTarget, ToDo, ToDoQueryTarget},
};
use actix_web::{
    get,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};
use std::collections::{BTreeSet, HashSet};

pub type ReadListsRequest = HashSet<ListID>;
pub type ReadSetsRequest = HashSet<SetQueryTarget>;
pub type ReadToDosRequest = HashSet<ToDoQueryTarget>;

pub type ReadListsResponse = BTreeSet<List>;
pub type ReadSetsResponse = BTreeSet<Set>;
pub type ReadToDosResponse = BTreeSet<ToDo>;

#[get("/api/lists")]
pub async fn read_lists(
    req: MaybeJson<ReadListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<ReadListsResponse>, JsonError> {
    query_all_or_some(req, db_conn_pool, query_all_lists, query_lists).await
}

#[get("/api/sets")]
pub async fn read_sets(
    req: MaybeJson<ReadSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<ReadSetsResponse>, JsonError> {
    query_all_or_some(req, db_conn_pool, query_all_sets, query_sets).await
}

#[get("/api/to_dos")]
pub async fn read_to_dos(
    req: MaybeJson<ReadToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<ReadToDosResponse>, JsonError> {
    query_all_or_some(req, db_conn_pool, query_all_todos, query_todos).await
}
