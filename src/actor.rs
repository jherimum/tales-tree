use anyhow::bail;
use sqlx::Type;

use crate::{id::Id, storage::model::user::User};

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "actor_type", rename_all = "snake_case")]
pub enum ActorType {
    User,
    System,
}

#[derive(Debug, Clone)]
pub enum Actor {
    User(User),
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

impl From<&Actor> for Option<Id> {
    fn from(value: &Actor) -> Self {
        match value {
            Actor::User(user) => Some(*user.id()),
            Actor::System => None,
        }
    }
}

impl Actor {
    pub fn is_user(&self) -> bool {
        matches!(self, Actor::User(_))
    }
}

impl TryFrom<&Actor> for User {
    type Error = anyhow::Error;

    fn try_from(value: &Actor) -> Result<Self, Self::Error> {
        match value {
            Actor::User(u) => Ok(u.clone()),
            _ => bail!("Actor is not a user"),
        }
    }
}
