use sqlx::{Column, Error as SQLXError, Row, Sqlite, pool::PoolConnection};

use crate::types::{ListID, SetAddress, ToDoAddress};

pub async fn delete_lists(
    mut db_conn: PoolConnection<Sqlite>,
    adds: Vec<ListID>,
) -> Result<(), SQLXError> {
    let query = format!(
        "DELETE FROM lists WHERE lid IN ({});",
        adds.into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    for row in query_result {
        for col in row.columns() {
            println!("Column: {}", col.name());
        }
    }

    Ok(())
}

pub async fn delete_sets(
    mut db_conn: PoolConnection<Sqlite>,
    adds: Vec<SetAddress>,
) -> Result<(), SQLXError> {
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
        "DELETE FROM sets WHERE lid IN ({}) OR (lid, sid) IN ({});",
        whole_list_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    for row in query_result {
        for col in row.columns() {
            println!("Column: {}", col.name());
        }
    }

    Ok(())
}

pub async fn delete_todos(
    mut db_conn: PoolConnection<Sqlite>,
    adds: Vec<ToDoAddress>,
) -> Result<(), SQLXError> {
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
        "DELETE FROM todos WHERE lid IN ({}) OR (lid, sid) IN ({}) OR (lid, sid, tdid) IN ({});",
        whole_list_ids, whole_set_ids, singular_ids
    );

    let query_result = sqlx::query(query.as_str()).fetch_all(&mut *db_conn).await?;

    for row in query_result {
        for col in row.columns() {
            println!("Column: {}", col.name());
        }
    }

    Ok(())
}
