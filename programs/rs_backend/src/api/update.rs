use actix_web::{
    HttpResponse, Responder, patch,
    web::{Data, Json},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::{
    db::sqlx::{
        update_lists as db_update_lists, update_sets as db_update_sets,
        update_todos as db_update_todos,
    },
    types::{ListID, SetID, ToDoID},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateList {
    pub list_id: ListID,
    pub title: Option<String>,
}
type UpdateListsRequest = Vec<UpdateList>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSet {
    pub set_id: SetID,
    pub list_id: ListID,
    pub title: Option<String>,
}
type UpdateSetsRequest = Vec<UpdateSet>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateToDo {
    pub to_do_id: ToDoID,
    pub set_id: Option<SetID>,
    pub list_id: ListID,
    pub title: Option<String>,
    pub complete: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}
type UpdateToDosRequest = Vec<UpdateToDo>;

// single SQL query, container level updates.

#[patch("/api/lists")]
pub async fn update_lists(
    req: Json<UpdateListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    db_update_lists(db_conn_pool, req.into_inner())
        .await
        .unwrap();
    HttpResponse::Accepted().finish()
}

#[patch("/api/sets")]
pub async fn update_sets(
    req: Json<UpdateSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    db_update_sets(db_conn_pool, req.into_inner())
        .await
        .unwrap();
    HttpResponse::Accepted().finish()
}

#[patch("/api/to_dos")]
pub async fn update_to_dos(
    req: Json<UpdateToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    db_update_todos(db_conn_pool, req.into_inner())
        .await
        .unwrap();
    HttpResponse::Accepted().finish()
}
