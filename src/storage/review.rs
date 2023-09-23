use crate::{id::Id, DateTime};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Builder, Clone, FromRow, Getters)]
#[builder(setter(into))]
pub struct Review {
    id: Id,
    fragment_id: Id,
    reviewer_id: Id,
    action: ReviewAction,
    comment: Option<String>,
    created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, Copy)]
#[sqlx(type_name = "review_action", rename_all = "snake_case")]
pub enum ReviewAction {
    Approve,
    Reject,
    RequestChanges,
}
