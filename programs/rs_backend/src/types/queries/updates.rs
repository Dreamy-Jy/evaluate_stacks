use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{ListID, SetID, SetQueryTarget, ToDoQueryTarget};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateList {
    pub list_id: ListID,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSet {
    pub target: SetQueryTarget,
    pub list_id: Option<ListID>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateToDo {
    pub target: ToDoQueryTarget,
    pub set_id: Option<SetID>,
    pub list_id: Option<ListID>,
    pub title: Option<String>,
    pub complete: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}
