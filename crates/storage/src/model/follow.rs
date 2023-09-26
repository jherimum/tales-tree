use commons::{clock::DateTime, id::Id};
use derive_builder::Builder;
use derive_getters::Getters;
use sqlx::FromRow;

use crate::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRow, Builder, Getters)]
#[builder(setter(into))]
pub struct Follow {
    follower_id: Id,
    followee_id: Id,
    created_at: DateTime,
}

impl Entity for Follow {
    type Id = (Id, Id);

    fn id(&self) -> Self::Id {
        (self.follower_id, self.followee_id)
    }
}
