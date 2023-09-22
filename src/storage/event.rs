use super::StorageError;
use crate::{
    events::{Event as EventTrait, EventType},
    id::Id,
    DateTime,
};
use derive_getters::Getters;
use serde::de::DeserializeOwned;
use serde_json::Value;
use sqlx::{FromRow, PgExecutor, Type};

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

impl DbEvent {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO events (id, event_type, event_data, timestamp)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.event_type)
        .bind(self.event_data)
        .bind(self.timestamp)
        .fetch_one(exec)
        .await?)
    }

    pub async fn all<'e, E: PgExecutor<'e>>(exec: E) -> Result<Vec<Self>, StorageError> {
        Ok(sqlx::query_as(
            r#"
            SELECT * FROM events
            "#,
        )
        .fetch_all(exec)
        .await?)
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
