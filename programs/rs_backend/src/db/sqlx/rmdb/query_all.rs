use std::collections::BTreeSet;

use actix_web::web::Data;
use sqlx::{Error as SQLXError, Pool, Row, Sqlite};

use crate::types::{List, Set, ToDo};

pub async fn query_all_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<BTreeSet<List>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query_result = sqlx::query("SELECT * FROM lists")
        .fetch_all(&mut *db_conn)
        .await?;

    let mut lists = BTreeSet::new();
    for row in query_result {
        let list = List {
            id: row.get("id"),
            title: row.get("title"),
        };
        lists.insert(list);
    }

    Ok(lists)
}

pub async fn query_all_sets(db_conn_pool: Data<Pool<Sqlite>>) -> Result<BTreeSet<Set>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query_result = sqlx::query("SELECT * FROM sets")
        .fetch_all(&mut *db_conn)
        .await?;

    let mut sets = BTreeSet::new();
    for row in query_result {
        let set = Set {
            id: row.get("id"),
            list_id: row.get("list_id"),
            title: row.get("title"),
        };
        sets.insert(set);
    }

    Ok(sets)
}

pub async fn query_all_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
) -> Result<BTreeSet<ToDo>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query_result = sqlx::query("SELECT * FROM todos")
        .fetch_all(&mut *db_conn)
        .await?;

    let mut todos = BTreeSet::new();
    for row in query_result {
        let todo = ToDo {
            id: row.get("id"),
            list_id: row.get("list_id"),
            set_id: row.get("set_id"),
            title: row.get("title"),
            complete: row.get("complete"),
            due_date: row.get("due_date"),
        };
        todos.insert(todo);
    }

    Ok(todos)
}
