use actix_web::{
    get,
    web::{Data, Json},
};
use sqlx::{Pool, Sqlite};

use crate::{
    db::sqlx::{
        query_all_lists, query_all_sets, query_all_todos, query_lists, query_sets, query_todos,
    },
    types::{List, ListID, Set, SetAddress, ToDo, ToDoAddress},
};

type ReadListsRequest = Vec<ListID>;
type ReadSetsRequest = Vec<SetAddress>;
type ReadToDosRequest = Vec<ToDoAddress>;

type ReadListsResponse = Vec<List>;
type ReadSetsResponse = Vec<Set>;
type ReadToDosResponse = Vec<ToDo>;

#[get("/api/lists")]
pub async fn read_lists(
    req: Option<Json<ReadListsRequest>>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Json<ReadListsResponse> {
    let lists: Vec<List>;

    match req {
        None => {
            lists = query_all_lists(db_conn_pool).await.unwrap();
        }
        Some(Json(r)) if r.len() == 0 || r.is_empty() => {
            lists = query_all_lists(db_conn_pool).await.unwrap();
        }
        Some(Json(r)) => {
            lists = query_lists(db_conn_pool, r).await.unwrap();
        }
    }

    Json(lists)
}

#[get("/api/sets")]
pub async fn read_sets(
    req: Option<Json<ReadSetsRequest>>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Json<ReadSetsResponse> {
    let sets: Vec<Set>;

    match req {
        None => {
            sets = query_all_sets(db_conn_pool).await.unwrap();
        }
        Some(Json(r)) if r.len() == 0 || r.is_empty() => {
            sets = query_all_sets(db_conn_pool).await.unwrap();
        }
        Some(Json(r)) => {
            sets = query_sets(db_conn_pool, r).await.unwrap();
        }
    }

    Json(sets)
}

#[get("/api/to_dos")]
pub async fn read_to_dos(
    req: Option<Json<ReadToDosRequest>>,
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Json<ReadToDosResponse> {
    let todos: Vec<ToDo>;

    match req {
        None => {
            todos = query_all_todos(db_conn_pool).await.unwrap();
        }
        Some(Json(r)) if r.len() == 0 || r.is_empty() => {
            todos = query_all_todos(db_conn_pool).await.unwrap();
        }
        Some(Json(r)) => {
            todos = query_todos(db_conn_pool, r).await.unwrap();
        }
    }

    Json(todos)
}
