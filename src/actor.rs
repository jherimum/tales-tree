use anyhow::bail;
use sqlx::Type;

use crate::{id::Id, storage::user::User};

#[derive(Debug, Clone, Type)]
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
        match self {
            Actor::User(_) => true,
            _ => false,
        }
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
