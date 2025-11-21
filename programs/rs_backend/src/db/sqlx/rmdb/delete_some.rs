use std::collections::HashSet;

use actix_web::web::Data;
use sqlx::{
    Error::{self as SQLXError, InvalidArgument},
    Pool, Row, Sqlite,
};

use crate::{
    api::{
        DeleteListsRequest, DeleteListsResponse, DeleteSetsRequest, DeleteSetsResponse,
        DeleteToDosRequest, DeleteToDosResponse,
    },
    types::{SetQueryTarget, ToDoQueryTarget},
};

pub async fn delete_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: DeleteListsRequest,
) -> Result<DeleteListsResponse, SQLXError> {
    if adds.len() == 0 {
        return Err(InvalidArgument(
            "Caller Provided no entries to the database".to_string(),
        ));
    }

    let mut db_conn = db_conn_pool.acquire().await?;

    let query = format!(
        "DELETE FROM Lists WHERE id IN ({}) RETURNING id;",
        adds.into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    let mut deleted_ids = HashSet::new();
    for row in query_result {
        deleted_ids.insert(row.get("id"));
    }

    Ok(deleted_ids)
}

pub async fn delete_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: DeleteSetsRequest,
) -> Result<DeleteSetsResponse, SQLXError> {
    if adds.len() == 0 {
        return Err(InvalidArgument(
            "Caller Provided no entries to the database".to_string(),
        ));
    }

    let mut db_conn = db_conn_pool.acquire().await?;

    let (whole_list_ids, singular_ids) = {
        let mut acc =
            adds.into_iter()
                .fold((String::new(), String::new()), |(mut wl, mut s), ele| {
                    match ele {
                        SetQueryTarget::List(list_id) => {
                            wl.push_str(&list_id.to_string());
                            wl.push_str(", ");
                        }
                        SetQueryTarget::Set(set_id) => {
                            s.push_str(&set_id.to_string());
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
        "DELETE FROM Sets WHERE list_id IN ({}) OR id IN ({}) RETURNING id;",
        whole_list_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    let mut deleted_ids = HashSet::new();
    for row in query_result {
        deleted_ids.insert(row.get("id"));
    }

    Ok(deleted_ids)
}

pub async fn delete_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: DeleteToDosRequest,
) -> Result<DeleteToDosResponse, SQLXError> {
    if adds.len() == 0 {
        return Err(InvalidArgument(
            "Caller Provided no entries to the database".to_string(),
        ));
    }

    let mut db_conn = db_conn_pool.acquire().await?;

    let (whole_list_ids, whole_set_ids, singular_ids) = {
        let mut acc = adds.into_iter().fold(
            (String::new(), String::new(), String::new()),
            |(mut wl, mut ws, mut s), ele| {
                match ele {
                    ToDoQueryTarget::List(list_id) => {
                        wl.push_str(&list_id.to_string());
                        wl.push_str(", ");
                    }
                    ToDoQueryTarget::Set(set_id) => {
                        ws.push_str(&set_id.to_string());
                        ws.push_str(", ");
                    }
                    ToDoQueryTarget::ToDo(todo_id) => {
                        s.push_str(&todo_id.to_string());
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
        "DELETE FROM Todos WHERE list_id IN ({}) OR set_id IN ({}) OR id IN ({}) RETURNING id;",
        whole_list_ids, whole_set_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    let mut deleted_ids = HashSet::new();
    for row in query_result {
        deleted_ids.insert(row.get("id"));
    }

    Ok(deleted_ids)
}
