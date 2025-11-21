use std::collections::BTreeSet;

use actix_web::{
    put,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};

use crate::{
    api::{
        UpdateList, UpdateSet, UpdateToDo,
        types::{JsonError, MaybeJson},
        utils::query_some,
    },
    db::sqlx::{
        update_lists as db_update_lists, update_sets as db_update_sets,
        update_todos as db_update_todos,
    },
    types::{List, Set, ToDo},
};

type UpdateListsRequest = Vec<UpdateList>;
type UpdateSetsRequest = Vec<UpdateSet>;
type UpdateToDosRequest = Vec<UpdateToDo>;

type UpdateListsResponse = BTreeSet<List>;
type UpdateSetsResponse = BTreeSet<Set>;
type UpdateToDoResponse = BTreeSet<ToDo>;

#[put("/api/lists")]
pub async fn update_lists(
    req: MaybeJson<UpdateListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<UpdateListsResponse>, JsonError> {
    query_some(req, db_conn_pool, db_update_lists).await
}

#[put("/api/sets")]
pub async fn update_sets(
    req: MaybeJson<UpdateSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<UpdateSetsResponse>, JsonError> {
    query_some(req, db_conn_pool, db_update_sets).await
}

#[put("/api/to_dos")]
pub async fn update_to_dos(
    req: MaybeJson<UpdateToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<Json<UpdateToDoResponse>, JsonError> {
    query_some(req, db_conn_pool, db_update_todos).await
}
