use ::serde::de::DeserializeOwned;
use commons::{actor::ActorType, clock::DateTime, commands::CommandType, id::Id};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, Type};

use crate::Entity;

#[derive(Debug, FromRow, Getters, Builder)]
pub struct Task {
    id: Id,
    command_type: CommandType,
    commnad_data: CommandData,
    actor_type: ActorType,
    actor_id: Option<Id>,
    created_at: DateTime,
    scheduled_at: DateTime,
    completed_at: Option<DateTime>,
}

impl Entity for Task {
    type Id = Id;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[derive(Debug, Type, Clone)]
#[sqlx(transparent)]
pub struct CommandData(Value);

impl<C: Serialize> From<C> for CommandData {
    fn from(value: C) -> Self {
        Self(serde_json::to_value(value).unwrap())
    }
}

impl CommandData {
    pub fn into_command<T: DeserializeOwned>(self) -> T {
        serde_json::from_value(self.0).unwrap()
    }
}
