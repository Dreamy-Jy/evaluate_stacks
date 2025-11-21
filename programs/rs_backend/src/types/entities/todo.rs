use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::types::{ListID, SetID, ToDoID};

#[derive(Serialize, Debug)]
pub struct ToDo {
    pub id: ToDoID,
    pub set_id: Option<SetID>,
    pub list_id: ListID,
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
