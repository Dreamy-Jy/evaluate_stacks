use actix_web::web::Data;
use sqlx::{Error as SQLXError, Pool, Sqlite};

use crate::api::{UpdateList, UpdateSet, UpdateToDo};

pub async fn update_lists(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: Vec<UpdateList>,
) -> Result<(), SQLXError> {
    let mut transaction = db_conn_pool.begin().await?;

    for update in mods {
        let updates;
        let address;

        if let Some(title) = update.title {
            updates = format!("title = '{}'", title);
            address = format!("lid = {}", update.list_id);
            let query = format!("UPDATE lists SET {} WHERE {};", updates, address);

            sqlx::query(query.as_str())
                .fetch_all(&mut *transaction)
                .await?;
        }

        continue;
    }

    transaction.commit().await?;
    Ok(())
}

pub async fn update_sets(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: Vec<UpdateSet>,
) -> Result<(), SQLXError> {
    let mut transaction = db_conn_pool.begin().await?;

    for update in mods {
        let updates;
        let address;

        if let Some(title) = update.title {
            updates = format!("title = '{}'", title);
            address = format!("lid = {} AND sid = {}", update.list_id, update.set_id);
            let query = format!("UPDATE sets SET {} WHERE {};", updates, address);

            sqlx::query(query.as_str())
                .fetch_all(&mut *transaction)
                .await?;
        }

        continue;
    }

    transaction.commit().await?;
    Ok(())
}

pub async fn update_todos(
    db_conn_pool: Data<Pool<Sqlite>>,
    mods: Vec<UpdateToDo>,
) -> Result<(), SQLXError> {
    let mut transaction = db_conn_pool.begin().await?;

    for update in mods {
        let mut updates: String = String::new();
        let address;

        if let Some(title) = update.title {
            updates.push_str(format!("title = '{}', ", title).as_str());
        }
        if let Some(complete) = update.complete {
            updates.push_str(format!("complete = {}, ", complete).as_str());
        }
        if let Some(due_date) = update.due_date {
            updates.push_str(format!("due_date = '{}', ", due_date.to_rfc3339()).as_str());
        }
        updates = updates.trim_end_matches(", ").to_string();

        if updates.len() == 0 {
            continue;
        }

        let sid_str = match update.set_id {
            Some(sid) => sid.to_string(),
            None => "NULL".to_string(),
        };

        address = format!(
            "lid = {} AND sid = {} AND tdid = {}",
            update.list_id, sid_str, update.to_do_id
        );

        let query = format!("UPDATE todos SET {} WHERE {};", updates, address);

        sqlx::query(query.as_str())
            .fetch_all(&mut *transaction)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}
