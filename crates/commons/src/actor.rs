use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::id::Id;

pub trait ActorTrait: Debug + Send + Sync {
    fn id(&self) -> Option<Id> {
        self.actor().id()
    }
    fn actor_type(&self) -> ActorType {
        (&self.actor()).into()
    }
    fn actor(&self) -> Actor;
}

#[derive(Debug, Clone, Type, PartialEq, Eq)]
#[sqlx(type_name = "actor_type", rename_all = "snake_case")]
pub enum ActorType {
    User,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum Actor {
    User(Id),
    System,
}

impl From<&Actor> for ActorType {
    fn from(value: &Actor) -> Self {
        match value {
            Actor::User(_) => Self::User,
            Actor::System => Self::System,
        }
    }
}

impl Actor {
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User(_))
    }

    pub const fn id(&self) -> Option<Id> {
        match self {
            Self::User(id) => Some(*id),
            Self::System => None,
        }
    }
}
