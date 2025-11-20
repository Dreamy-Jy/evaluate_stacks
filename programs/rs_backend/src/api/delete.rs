use actix_web::{
    Error, HttpResponse, Responder, delete,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};

use crate::{
    db::sqlx::{delete_lists as db_delete_lists, delete_sets as db_delete_sets, delete_todos as db_delete_todos},
    types::{ListID, SetAddress, ToDoAddress},
};

type DeleteListsRequest = Vec<ListID>;
type DeleteSetsRequest = Vec<SetAddress>;
type DeleteToDosRequest = Vec<ToDoAddress>;

#[delete("/api/lists/delete")]
pub async fn delete_lists(
    req: Result<Json<DeleteListsRequest>, Error>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    let conn = db_conn_pool.acquire().await.unwrap();

    match req {
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid request: {}", e)),
        Ok(Json(_data)) => {
            db_delete_lists(conn, _data).await.unwrap()
        },
    }

    HttpResponse::Ok().finish()
}

#[delete("/api/sets/delete")]
pub async fn delete_sets(
    req: Result<Json<DeleteSetsRequest>, Error>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    let conn = db_conn_pool.acquire().await.unwrap();

    match req {
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid request: {}", e)),
        Ok(Json(_data)) => {
            db_delete_sets(conn, _data).await.unwrap()
        },
    }

    HttpResponse::Ok().finish()
}

#[delete("/api/to_dos/delete")]
pub async fn delete_to_dos(
    req: Result<Json<DeleteToDosRequest>, Error>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> impl Responder {
    let conn = db_conn_pool.acquire().await.unwrap();

    match req {
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid request: {}", e)),
        Ok(Json(_data)) => {
            db_delete_todos(conn, _data).await.unwrap()
        },
    }

    HttpResponse::Ok().finish()
}
