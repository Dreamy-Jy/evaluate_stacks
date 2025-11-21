use std::collections::BTreeSet;

use actix_web::web::Data;
use sqlx::{Error as SQLXError, Pool, Row, Sqlite};

use crate::{
    api::{
        UpdateListsRequest, UpdateListsResponse, UpdateSetsRequest, UpdateSetsResponse,
        UpdateToDoResponse, UpdateToDosRequest,
    },
    types::{List, Set, SetQueryTarget, ToDo, ToDoQueryTarget},
};

pub async fn update_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: UpdateListsRequest,
) -> Result<UpdateListsResponse, SQLXError> {
    if mods.is_empty() {
        return Err(SQLXError::InvalidArgument(
            "Can't have zero modification when running update on Lists Table.".to_string(),
        ));
    }

    let mut transaction = db_conn_pool.begin().await?;
    let mut output = BTreeSet::new();

    for update in mods {
        let updates;
        let address;

        updates = format!("title = '{}'", update.title);
        address = format!("id = {}", update.list_id);
        let query = format!(
            "UPDATE Lists SET {} WHERE {} RETURNING * ;",
            updates, address
        );

        let query_result = sqlx::query(query.as_str())
            .fetch_all(&mut *transaction)
            .await?;

        for row in query_result {
            output.replace(List {
                id: row.get("id"),
                title: row.get("title"),
            });
        }
    }

    transaction.commit().await?;
    Ok(output)
}

pub async fn update_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: UpdateSetsRequest,
) -> Result<UpdateSetsResponse, SQLXError> {
    if mods.is_empty() {
        return Err(SQLXError::InvalidArgument(
            "Can't have zero modification when running update on Sets Table.".to_string(),
        ));
    }

    let mut transaction = db_conn_pool.begin().await?;

    let mut output = BTreeSet::new();

    for update in mods {
        let mut updates = String::new();
        match update.list_id {
            Some(id) => updates.push_str(format!("{}, ", id).as_str()),
            None => updates.push_str("list_id, "),
        }
        match update.title {
            Some(title) => updates.push_str(format!("'{}', ", title).as_str()),
            None => updates.push_str("title, "),
        }
        updates = updates.trim_end_matches(", ").to_string();

        let mut target = String::new();
        match update.target {
            SetQueryTarget::List(id) => target.push_str(format!("list_id = {}", id).as_str()),
            SetQueryTarget::Set(id) => target.push_str(format!("id = {}", id).as_str()),
        }

        let query = format!(
            "UPDATE Sets SET (list_id, title) = ({}) WHERE {} RETURNING * ;",
            updates, target
        );

        let query_result = sqlx::query(query.as_str())
            .fetch_all(&mut *transaction)
            .await?;

        for row in query_result {
            output.replace(Set {
                id: row.get("id"),
                list_id: row.get("list_id"),
                title: row.get("title"),
            });
        }
    }

    transaction.commit().await?;
    Ok(output)
}

pub async fn update_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: UpdateToDosRequest,
) -> Result<UpdateToDoResponse, SQLXError> {
    if mods.is_empty() {
        return Err(SQLXError::InvalidArgument(
            "Can't have zero modification when running update on Todos Table.".to_string(),
        ));
    }

    let mut transaction = db_conn_pool.begin().await?;

    let mut output = BTreeSet::new();

    for update in mods {
        let mut updates: String = String::new();
        match update.list_id {
            Some(id) => updates.push_str(format!("{}, ", id).as_str()),
            None => updates.push_str("list_id, "),
        }
        match update.set_id {
            Some(id) => updates.push_str(format!("{}, ", id).as_str()),
            None => updates.push_str("set_id, "),
        }
        match update.title {
            Some(t) => updates.push_str(format!("'{}', ", t).as_str()),
            None => updates.push_str("title, "),
        }
        match update.complete {
            Some(c) => updates.push_str(format!("{}, ", c).as_str()),
            None => updates.push_str("complete, "),
        }
        match update.due_date {
            Some(d) => updates.push_str(format!("'{}', ", d.to_rfc3339()).as_str()),
            None => updates.push_str("due_date, "),
        }
        updates = updates.trim_end_matches(", ").to_string();

        let mut target = String::new();
        match update.target {
            ToDoQueryTarget::List(id) => target.push_str(format!("list_id = {}", id).as_str()),
            ToDoQueryTarget::Set(id) => target.push_str(format!("set_id = {}", id).as_str()),
            ToDoQueryTarget::ToDo(id) => target.push_str(format!("id = {}", id).as_str()),
        }

        let query = format!(
            "UPDATE Todos SET (list_id, set_id, title, complete, due_date) = ({}) WHERE {} RETURNING * ;",
            updates, target
        );

        let query_result = sqlx::query(query.as_str())
            .fetch_all(&mut *transaction)
            .await?;

        for row in query_result {
            output.replace(ToDo {
                id: row.get("id"),
                list_id: row.get("list_id"),
                set_id: row.get("set_id"),
                complete: row.get("complete"),
                due_date: row.get("due_date"),
                title: row.get("title"),
            });
        }
    }

    transaction.commit().await?;

    Ok(output)
}
