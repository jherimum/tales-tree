pub mod actor;
pub mod commands;
pub mod id;
pub mod storage;

use derive_getters::Getters;
use id::Id;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Getters, Clone)]
pub struct User {
    id: Id,
}

impl Into<Id> for User {
    fn into(self) -> Id {
        self.id
    }
}

pub struct Review {
    id: Id,
    tale: Id,
    reviewer: Id,
    comment: Option<String>,
    action: ReviewAction,
    created_at: DateTime,
}

pub struct Comment {
    id: Id,
    review: Id,
    user: Id,
    comment: String,
    created_at: DateTime,
}

pub enum ReviewAction {
    Approve,
    Reject,
    RequestChanges,
}

//social
pub struct Watch {
    id: Id,
    user: Id,
    tale: Id,
}

pub struct Like {
    id: Id,
    user: Id,
    tale: Id,
}

pub struct Follow {
    id: Id,
    follower: Id,
    followee: Id,
}
