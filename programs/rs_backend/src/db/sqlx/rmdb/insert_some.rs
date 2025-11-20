use actix_web::web::Data;
use sqlx::{
    Error::{self as SQLXError, InvalidArgument},
    Pool, Sqlite,
};

use crate::api::{CreateList, CreateSet, CreateToDo};

pub async fn insert_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    entries: Vec<CreateList>,
) -> Result<(), SQLXError> {
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

    let query = format!("INSERT INTO lists (lid, title) VALUES {};", values);

    let mut db_conn = db_conn_pool.acquire().await?;
    sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    Ok(())
}

pub async fn insert_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    entries: Vec<CreateSet>,
) -> Result<(), SQLXError> {
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

    let query = format!("INSERT INTO sets (lid, title) VALUES {};", values);

    let mut db_conn = db_conn_pool.acquire().await?;
    sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    Ok(())
}

pub async fn insert_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
    entries: Vec<CreateToDo>,
) -> Result<(), SQLXError> {
    if entries.len() == 0 {
        return Err(InvalidArgument(
            "Caller Provided no entries to the database".to_string(),
        ));
    }

    let values = entries
        .iter()
        .fold(String::new(), |acc, ele| {
            let sid_str = match ele.set_id {
                Some(sid) => sid.to_string(),
                None => "NULL".to_string(),
            };

            let due_date_str = match ele.due_date {
                Some(due_date) => format!("'{}'", due_date.to_rfc3339()),
                None => "NULL".to_string(),
            };

            let complete_str = match ele.complete {
                Some(complete) => complete.to_string(),
                None => false.to_string(),
            };

            acc + &format!(
                "({}, {}, '{}', {}, {}), ",
                ele.list_id, sid_str, ele.title, complete_str, due_date_str
            )
        })
        .trim_end_matches(", ")
        .to_string();

    let query = format!(
        "INSERT INTO todos (lid, sid, title, complete, due_date) VALUES {};",
        values
    );

    let mut db_conn = db_conn_pool.acquire().await?;
    sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    Ok(())
}
