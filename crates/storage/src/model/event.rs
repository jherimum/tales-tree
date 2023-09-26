use commons::{clock::DateTime, events::EventType, id::Id};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Type};

use crate::Entity;

#[derive(Debug, Clone, Type)]
#[sqlx(transparent)]
pub struct EventData(Value);

impl EventData {
    pub fn into_event<T: DeserializeOwned>(&self) -> T {
        serde_json::from_value(self.0.clone()).unwrap()
    }
}

impl<S: Serialize> From<S> for EventData {
    fn from(value: S) -> Self {
        Self(serde_json::to_value(value).unwrap())
    }
}

#[derive(Debug, Clone, FromRow, Getters, Builder)]
pub struct DbEvent {
    id: Id,
    event_type: EventType,
    event_data: EventData,
    timestamp: DateTime,
}

impl Entity for DbEvent {
    type Id = Id;
    fn id(&self) -> Id {
        self.id
    }
}
