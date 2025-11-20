use actix_web::web::Data;
use sqlx::{Error as SQLXError, Pool, Row, Sqlite};

use crate::types::{List, ListID, Set, SetAddress, ToDo, ToDoAddress};

pub async fn query_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: Vec<ListID>,
) -> Result<Vec<List>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let query = format!(
        "SELECT * FROM lists WHERE lid IN ({});",
        adds.into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

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

pub async fn query_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: Vec<SetAddress>,
) -> Result<Vec<Set>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let (whole_list_ids, singular_ids) = {
        let mut acc =
            adds.into_iter()
                .fold((String::new(), String::new()), |(mut wl, mut s), ele| {
                    match ele {
                        SetAddress::WholeList(lid) => {
                            wl.push_str(&lid.to_string());
                            wl.push_str(", ");
                        }
                        SetAddress::Singular(lid, sid) => {
                            s.push_str(&format!("({}, {})", lid, sid));
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
        "SELECT * FROM sets WHERE lid IN ({}) OR (lid, sid) IN ({});",
        whole_list_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

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

pub async fn query_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
    adds: Vec<ToDoAddress>,
) -> Result<Vec<ToDo>, SQLXError> {
    let mut db_conn = db_conn_pool.acquire().await?;

    let (whole_list_ids, whole_set_ids, singular_ids) = {
        let mut acc = adds.into_iter().fold(
            (String::new(), String::new(), String::new()),
            |(mut wl, mut ws, mut s), ele| {
                match ele {
                    ToDoAddress::WholeList(lid) => {
                        wl.push_str(&lid.to_string());
                        wl.push_str(", ");
                    }
                    ToDoAddress::WholeSet(lid, sid) => {
                        ws.push_str(&format!("({}, {})", lid, sid));
                        ws.push_str(", ");
                    }
                    ToDoAddress::Singular(lid, sid, tdid) => {
                        let sid_str: String = match sid {
                            Some(sid) => sid.to_string(),
                            None => "NULL".to_string(),
                        };

                        s.push_str(&format!("({}, {}, {})", lid, sid_str, tdid));
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
        "SELECT * FROM todos WHERE lid IN ({}) OR (lid, sid) IN ({}) OR (lid, sid, tdid) IN ({});",
        whole_list_ids, whole_set_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

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
