use actix_web::web::Data;
use sqlx::{Error as SQLXError, Pool, Row, Sqlite};

use crate::types::{List, Set, ToDo};

pub async fn query_all_lists(db_conn_pool: Data<Pool<Sqlite>>) -> Result<Vec<List>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query_result = sqlx::query("SELECT * FROM lists")
        .fetch_all(&mut *db_conn)
        .await?;

    let mut lists = Vec::new();
    for row in query_result {
        let list = List {
            id: row.get("lid"),
            title: row.get("title"),
        };
        lists.push(list);
    }

    Ok(lists)
}

pub async fn query_all_sets(db_conn_pool: Data<Pool<Sqlite>>) -> Result<Vec<Set>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query_result = sqlx::query("SELECT * FROM sets")
        .fetch_all(&mut *db_conn)
        .await?;

    let mut sets = Vec::new();
    for row in query_result {
        let set = Set {
            id: row.get("sid"),
            list_id: row.get("lid"),
            title: row.get("title"),
        };
        sets.push(set);
    }

    Ok(sets)
}

pub async fn query_all_todos(db_conn_pool: Data<Pool<Sqlite>>) -> Result<Vec<ToDo>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query_result = sqlx::query("SELECT * FROM todos")
        .fetch_all(&mut *db_conn)
        .await?;

    let mut todos = Vec::new();
    for row in query_result {
        let todo = ToDo {
            id: row.get("tdid"),
            list_id: row.get("lid"),
            set_id: row.get("sid"),
            title: row.get("title"),
            complete: row.get("complete"),
            due_date: row.get("due_date"),
        };
        todos.push(todo);
    }

    Ok(todos)
}
