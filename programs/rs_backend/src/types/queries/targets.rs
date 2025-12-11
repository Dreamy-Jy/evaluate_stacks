use serde::{Deserialize, Serialize};

use crate::types::{ListID, SetID, ToDoID};

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
