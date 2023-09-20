use anyhow::bail;

use crate::User;

#[derive(Debug, Clone)]
pub enum Actor {
    User(User),
    System,
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
