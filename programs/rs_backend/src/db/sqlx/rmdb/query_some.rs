use std::collections::{BTreeSet, HashSet};

use actix_web::web::Data;
use sqlx::{Error as SQLXError, Pool, Row, Sqlite};

use crate::types::{List, ListID, Set, SetQueryTarget, ToDo, ToDoQueryTarget};

pub async fn query_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: HashSet<ListID>,
) -> Result<BTreeSet<List>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query = format!(
        "SELECT * FROM Lists WHERE id IN ({});",
        adds.into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

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

pub async fn query_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: HashSet<SetQueryTarget>,
) -> Result<BTreeSet<Set>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let (whole_list_ids, singular_ids) = {
        let mut acc =
            adds.into_iter()
                .fold((String::new(), String::new()), |(mut wl, mut s), ele| {
                    match ele {
                        SetQueryTarget::List(id) => {
                            wl.push_str(&id.to_string());
                            wl.push_str(", ");
                        }
                        SetQueryTarget::Set(id) => {
                            s.push_str(&id.to_string());
                            s.push_str(", ");
                        }
                    }

                    (wl, s)
                });

        acc.0 = acc.0.trim_end_matches(", ").to_string();
        acc.1 = acc.1.trim_end_matches(", ").to_string();

        acc
    };

    let query = format!(
        "SELECT * FROM Sets WHERE list_id IN ({}) OR id IN ({});",
        whole_list_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

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

pub async fn query_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: HashSet<ToDoQueryTarget>,
) -> Result<BTreeSet<ToDo>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let (whole_list_ids, whole_set_ids, singular_ids) = {
        let mut acc = adds.into_iter().fold(
            (String::new(), String::new(), String::new()),
            |(mut wl, mut ws, mut s), ele| {
                match ele {
                    ToDoQueryTarget::List(id) => {
                        wl.push_str(&id.to_string());
                        wl.push_str(", ");
                    }
                    ToDoQueryTarget::Set(id) => {
                        ws.push_str(&id.to_string());
                        ws.push_str(", ");
                    }
                    ToDoQueryTarget::ToDo(id) => {
                        s.push_str(&id.to_string());
                        s.push_str(", ");
                    }
                }

                (wl, ws, s)
            },
        );

        acc.0 = acc.0.trim_end_matches(", ").to_string();
        acc.1 = acc.1.trim_end_matches(", ").to_string();
        acc.2 = acc.2.trim_end_matches(", ").to_string();

        acc
    };

    let query: String = format!(
        "SELECT * FROM todos WHERE list_id IN ({}) OR set_id IN ({}) OR id IN ({});",
        whole_list_ids, whole_set_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

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
