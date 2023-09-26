use crate::{id::Id, storage::Entity};
use derive_builder::Builder;
use derive_getters::Getters;
use sqlx::FromRow;

#[derive(Debug, Builder, Clone, FromRow, Getters)]
#[builder(setter(into))]
pub struct User {
    id: Id,
}

impl Entity for User {
    type Id = Id;

    fn id(&self) -> Self::Id {
        self.id
    }
}
