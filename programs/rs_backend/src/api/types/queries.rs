use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{ListID, SetID};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateList {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSet {
    pub list_id: ListID,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateToDo {
    pub list_id: ListID,
    pub set_id: Option<SetID>,
    pub title: String,
    pub complete: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}
