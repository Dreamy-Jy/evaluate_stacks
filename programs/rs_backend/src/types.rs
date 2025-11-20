use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct List {
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Debug)]
pub struct Set {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
}

#[derive(Serialize, Debug)]
pub struct ToDo {
    pub id: i32,
    pub set_id: Option<i32>,
    pub list_id: i32,
    pub title: String,
    pub complete: bool,
    pub due_date: Option<DateTime<Utc>>,
}

pub type ListID = i32;
pub type SetID = i32;
pub type ToDoID = i32;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SetAddress {
    WholeList(ListID),
    Singular(ListID, SetID),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ToDoAddress {
    WholeList(ListID),
    WholeSet(ListID, SetID),
    Singular(ListID, Option<SetID>, ToDoID),
}
