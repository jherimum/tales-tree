use crate::{id::Id, DateTime};
use derive_builder::Builder;
use derive_getters::Getters;
use sqlx::FromRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRow, Builder, Getters)]
#[builder(setter(into))]
pub struct Follow {
    follower_id: Id,
    followee_id: Id,
    created_at: DateTime,
}
