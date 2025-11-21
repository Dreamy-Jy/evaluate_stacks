use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct List {
    pub id: i32,
    pub title: String,
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for List {}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for List {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Serialize, Debug)]
pub struct Set {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
}

impl PartialEq for Set {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Set {}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Set {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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

impl PartialEq for ToDo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ToDo {}

impl PartialOrd for ToDo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ToDo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for ToDo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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
