use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use serde::Serialize;

use crate::types::ListID;

#[derive(Serialize, Debug)]
pub struct List {
    pub id: ListID,
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
