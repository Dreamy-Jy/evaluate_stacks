use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, Eq, Hash)]
pub struct List {
    // I'll likely need to manually implement PartialEq, Eq, and Hash for this later
    pub id: i32,
    pub title: String,
}

#[derive(Serialize, Debug, PartialEq, Eq, Hash)]
pub struct Set {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
}

#[derive(Serialize, Debug, PartialEq, Eq, Hash)]
pub struct ToDo {
    pub id: i32,
    pub set_id: Option<i32>,
    pub list_id: i32,
    pub title: String,
    pub complete: bool,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SetAddress {
    WholeList(ListID),
    Singular(ListID, SetID),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ToDoAddress {
    WholeList(ListID),
    WholeSet(ListID, SetID),
    Singular(ListID, Option<SetID>, ToDoID),
}

pub type ListID = i32;
pub type SetID = i32;
pub type ToDoID = i32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "target", content = "id")]
pub enum SetQueryTarget {
    #[serde(rename = "list")]
    List(ListID),
    #[serde(rename = "set")]
    Set(SetID),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "target", content = "id")]
pub enum ToDoQueryTarget {
    #[serde(rename = "list")]
    List(ListID),
    #[serde(rename = "set")]
    Set(SetID),
    #[serde(rename = "todo")]
    ToDo(ToDoID),
}
