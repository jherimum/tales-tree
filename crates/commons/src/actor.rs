use sqlx::Type;

use crate::id::Id;

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "actor_type", rename_all = "snake_case")]
pub enum ActorType {
    User,
    System,
}

#[derive(Debug, Clone)]
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
    pub fn is_user(&self) -> bool {
        matches!(self, Actor::User(_))
    }

    pub fn id(&self) -> Option<Id> {
        match self {
            Actor::User(id) => Some(*id),
            Actor::System => None,
        }
    }
}
