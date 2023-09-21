pub mod actor;
pub mod commands;
pub mod id;
pub mod storage;

use chrono::NaiveDateTime;
use derive_getters::Getters;
use id::Id;

pub type DateTime = NaiveDateTime;

#[derive(Debug, Getters, Clone)]
pub struct User {
    id: Id,
}

impl Into<Id> for User {
    fn into(self) -> Id {
        self.id
    }
}
