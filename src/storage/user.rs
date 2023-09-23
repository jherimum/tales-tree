use crate::id::Id;
use derive_builder::Builder;
use derive_getters::Getters;
use sqlx::FromRow;

#[derive(Debug, Builder, Clone, FromRow, Getters)]
#[builder(setter(into))]
pub struct User {
    id: Id,
}
