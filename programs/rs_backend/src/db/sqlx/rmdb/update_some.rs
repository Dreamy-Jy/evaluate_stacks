use std::collections::BTreeSet;

use actix_web::web::Data;
use sqlx::{Error as SQLXError, Pool, Row, Sqlite};

use crate::{
    api::{UpdateList, UpdateSet, UpdateToDo},
    types::{List, Set, SetQueryTarget, ToDo, ToDoQueryTarget},
};

pub async fn update_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: Vec<UpdateList>,
) -> Result<BTreeSet<List>, SQLXError> {
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
            output.insert(List {
                id: row.get("id"),
                title: row.get("title"),
            });
        }

        continue;
    }

    transaction.commit().await?;
    Ok(output)
}

pub async fn update_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: Vec<UpdateSet>,
) -> Result<BTreeSet<Set>, SQLXError> {
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
            // Check for repeated IDs and
            // remove the previous entry
            // if it exists pop it and
            // insert the new entry
            output.insert(Set {
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
    mods: Vec<UpdateToDo>,
) -> Result<BTreeSet<ToDo>, SQLXError> {
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
            // Check for repeated IDs and
            // remove the previous entry
            // if it exists pop it and
            // insert the new entry
            output.insert(ToDo {
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

/*
let query_template =
    "UPDATE todos SET {} WHERE list_id IN ({}) OR set_id IN ({}) OR id IN ({}) RETURNING * ;";

let (list_id, set_id, title, complete, due_date) = (
    "list_id = CASE".to_string(),
    "set_id = CASE".to_string(),
    "title = CASE".to_string(),
    "complete = CASE".to_string(),
    "due_date = CASE".to_string(),
);

let (list_ids, set_ids, todo_ids) = (String::new(), String::new(), String::new());

for update in mods {
    match update.target {
        ToDoQueryTarget::List(id) => {
            list_id.push_str(format!("WHEN list_id = {}", id).as_str());
            set_id.push_str(format!("WHEN list_id = {}", id).as_str());
            title.push_str(format!("WHEN list_id = {}", id).as_str());
            complete.push_str(format!("WHEN list_id = {}", id).as_str());
            due_date.push_str(format!("WHEN list_id = {}", id).as_str());
            list_ids.push_str(format!("{}, ", id).as_str());
        }
        ToDoQueryTarget::Set(id) => {
            list_id.push_str(format!("WHEN set_id = {}", id).as_str());
            set_id.push_str(format!("WHEN set_id = {}", id).as_str());
            title.push_str(format!("WHEN set_id = {}", id).as_str());
            complete.push_str(format!("WHEN set_id = {}", id).as_str());
            due_date.push_str(format!("WHEN set_id = {}", id).as_str());
            set_ids.push_str(format!("{}, ", id).as_str());
        }
        ToDoQueryTarget::ToDo(id) => {
            list_id.push_str(format!("WHEN id = {}", id).as_str());
            set_id.push_str(format!("WHEN id = {}", id).as_str());
            title.push_str(format!("WHEN id = {}", id).as_str());
            complete.push_str(format!("WHEN id = {}", id).as_str());
            due_date.push_str(format!("WHEN id = {}", id).as_str());
            todo_ids.push_str(format!("{}, ", id).as_str());
        }
    }

    match update.list_id {
        Some(id) => list_id.push_str(format!("THEN {}", id).as_str()),
        None => list_id.push_str("THEN list_id"),
    }

    match update.set_id {
        Some(id) => set_id.push_str(format!("THEN {}", id).as_str()),
        None => set_id.push_str("THEN set_id"),
    }

    match update.complete {
        Some(b) => complete.push_str(format!("THEN {}", b).as_str()),
        None => complete.push_str("THEN complete"),
    }

    match update.due_date {
        Some(date) => due_date.push_str(format!("THEN '{}'", date.to_rfc3339()).as_str()),
        None => due_date.push_str("THEN due_date"),
    }

    match update.title {
        Some(t) => title.push_str(format!("THEN '{}'", t).as_str()),
        None => title.push_str("THEN title"),
    }
}

list_id.push_str("ELSE list_id END, ");
set_id.push_str("ELSE set_id END, ");
complete.push_str("ELSE complete END, ");
due_date.push_str("ELSE due_date END, ");
title.push_str("ELSE title END");
*/
