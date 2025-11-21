use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use serde::Serialize;

use crate::types::{ListID, SetID};

#[derive(Serialize, Debug)]
pub struct Set {
    pub id: SetID,
    pub list_id: ListID,
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
