use commons::{actor::ActorTrait, id::Id};
use derive_builder::Builder;
use derive_getters::Getters;
use sqlx::FromRow;

use crate::Entity;

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

impl ActorTrait for User {
    fn id(&self) -> Option<Id> {
        Some(self.id)
    }
    fn actor_type(&self) -> commons::actor::ActorType {
        commons::actor::ActorType::User
    }
}
