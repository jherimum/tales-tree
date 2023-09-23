use crate::{
    events::{Event as EventTrait, EventType},
    id::Id,
    storage::Entity,
    DateTime,
};
use derive_getters::Getters;
use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Type)]
#[sqlx(transparent)]
pub struct EventData(Value);

impl EventData {
    pub fn into_event<T: DeserializeOwned>(&self) -> T {
        serde_json::from_value(self.0.clone()).unwrap()
    }
}

#[derive(Debug, Clone, FromRow, Getters)]
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

impl<E: EventTrait> From<E> for DbEvent {
    fn from(value: E) -> Self {
        DbEvent {
            id: Id::new(),
            event_type: value.event_type(),
            event_data: EventData(serde_json::to_value(value.clone()).unwrap()),
            timestamp: value.timestamp(),
        }
    }
}
