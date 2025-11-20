use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::{
    db::sqlx::{insert_lists, insert_sets, insert_todos},
    types::{ListID, SetID},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateList {
    pub title: String,
}
type CreateListsRequest = Vec<CreateList>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSet {
    pub list_id: ListID,
    pub title: String,
}
type CreateSetsRequest = Vec<CreateSet>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateToDo {
    pub list_id: ListID,
    pub set_id: Option<SetID>,
    pub title: String,
    pub complete: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}
type CreateToDosRequest = Vec<CreateToDo>;

#[post("/api/lists")]
pub async fn create_lists(
    req: Json<CreateListsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    insert_lists(db_conn_pool, req.into_inner()).await.unwrap();

    HttpResponse::Accepted().finish()
}

#[post("/api/sets")]
pub async fn create_sets(
    req: Json<CreateSetsRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    insert_sets(db_conn_pool, req.into_inner()).await.unwrap();

    HttpResponse::Accepted().finish()
}

#[post("/api/to_dos")]
pub async fn create_to_dos(
    req: Json<CreateToDosRequest>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    insert_todos(db_conn_pool, req.into_inner()).await.unwrap();

    HttpResponse::Accepted().finish()
}
