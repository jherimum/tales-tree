use crate::{id::Id, storage::Entity, DateTime};
use derive_builder::Builder;
use derive_getters::Getters;
use sqlx::FromRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRow, Builder, Getters)]
#[builder(setter(into))]
pub struct Like {
    user_id: Id,
    fragment_id: Id,
    created_at: DateTime,
}

impl Entity for Like {
    type Id = (Id, Id);

    fn id(&self) -> Self::Id {
        (self.user_id, self.fragment_id)
    }
}