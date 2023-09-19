use crate::User;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ActorConversionError(&'static str);

pub enum Actor {
    User(User),
    System,
}

impl Into<Option<User>> for &Actor {
    fn into(self) -> Option<User> {
        match self {
            Actor::User(u) => Some(u.clone()),
            _ => None,
        }
    }
}
