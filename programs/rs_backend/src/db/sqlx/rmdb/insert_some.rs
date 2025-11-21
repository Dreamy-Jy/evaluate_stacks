use std::collections::HashSet;

use actix_web::web::Data;
use sqlx::{
    Error::{self as SQLXError, InvalidArgument},
    Pool, Row, Sqlite,
};

use crate::{
    api::{CreateList, CreateSet, CreateToDo},
    types::{List, Set, ToDo},
};

pub async fn insert_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    entries: Vec<CreateList>,
) -> Result<HashSet<List>, SQLXError> {
    if entries.len() == 0 {
        return Err(InvalidArgument(
            "Caller Provided no entries to the database".to_string(),
        ));
    }

    let values = entries
        .iter()
        .fold(String::new(), |acc, ele| {
            acc + &format!("('{}'), ", ele.title)
        })
        .trim_end_matches(", ")
        .to_string();

    let query = format!("INSERT INTO Lists (title) VALUES {} RETURNING *;", values);

    let mut db_conn = db_conn_pool.acquire().await?;
    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;
    let mut lists = HashSet::new();
    for row in query_result {
        let list = List {
            id: row.get("id"),
            title: row.get("title"),
        };
        lists.insert(list);
    }

    Ok(lists)
}

pub async fn insert_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    entries: Vec<CreateSet>,
) -> Result<HashSet<Set>, SQLXError> {
    if entries.len() == 0 {
        return Err(InvalidArgument(
            "Caller Provided no entries to the database".to_string(),
        ));
    }

    let values = entries
        .iter()
        .fold(String::new(), |acc, ele| {
            acc + &format!("({}, '{}'), ", ele.list_id, ele.title)
        })
        .trim_end_matches(", ")
        .to_string();

    let query = format!(
        "INSERT INTO Sets (list_id, title) VALUES {} RETURNING *;",
        values
    );

    let mut db_conn = db_conn_pool.acquire().await?;
    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;
    let mut sets = HashSet::new();
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

pub async fn insert_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
    entries: Vec<CreateToDo>,
) -> Result<HashSet<ToDo>, SQLXError> {
    if entries.len() == 0 {
        return Err(InvalidArgument(
            "Caller Provided no entries to the database".to_string(),
        ));
    }

    let values = entries
        .iter()
        .fold(String::new(), |acc, ele| {
            let set_id = match ele.set_id {
                Some(sid) => sid.to_string(),
                None => "NULL".to_string(),
            };

            let due_date = match ele.due_date {
                Some(due_date) => format!("'{}'", due_date.to_rfc3339()),
                None => "NULL".to_string(),
            };

            let complete = match ele.complete {
                Some(complete) => complete.to_string(),
                None => false.to_string(),
            };

            acc + &format!(
                "({}, {}, '{}', {}, {}), ",
                ele.list_id, set_id, ele.title, complete, due_date
            )
        })
        .trim_end_matches(", ")
        .to_string();

    let query = format!(
        "INSERT INTO Todos (list_id, set_id, title, complete, due_date) VALUES {} RETURNING *;",
        values
    );

    let mut db_conn = db_conn_pool.acquire().await?;
    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;
    let mut todos = HashSet::new();
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
