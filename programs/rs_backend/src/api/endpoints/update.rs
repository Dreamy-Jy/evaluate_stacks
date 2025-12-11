use std::collections::BTreeSet;

use actix_web::{
    put,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};

use crate::{
    api::{
        types::{JsonError, MaybeJson},
        utils::query_some,
    },
    db::sqlx::{
        update_lists as db_update_lists, update_sets as db_update_sets,
        update_todos as db_update_todos,
    },
    types::{List, Set, ToDo, UpdateList, UpdateSet, UpdateToDo},
};

pub type UpdateListsRequest = Vec<UpdateList>;
pub type UpdateSetsRequest = Vec<UpdateSet>;
pub type UpdateToDosRequest = Vec<UpdateToDo>;

pub type UpdateListsResponse = BTreeSet<List>;
pub type UpdateSetsResponse = BTreeSet<Set>;
pub type UpdateToDoResponse = BTreeSet<ToDo>;

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
