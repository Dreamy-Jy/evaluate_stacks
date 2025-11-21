use std::collections::BTreeSet;

use actix_web::{
    put,
    web::{Data, Json},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
    types::{List, ListID, Set, SetID, SetQueryTarget, ToDo, ToDoQueryTarget},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateList {
    pub list_id: ListID,
    pub title: String,
}
type UpdateListsRequest = Vec<UpdateList>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSet {
    pub target: SetQueryTarget,
    pub list_id: Option<ListID>,
    pub title: Option<String>,
}
type UpdateSetsRequest = Vec<UpdateSet>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateToDo {
    pub target: ToDoQueryTarget,
    pub set_id: Option<SetID>,
    pub list_id: Option<ListID>,
    pub title: Option<String>,
    pub complete: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}
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
