use super::StorageError;
use crate::{
    events::{Event as EventTrait, EventType},
    id::Id,
    DateTime,
};
use serde_json::Value;
use sqlx::{FromRow, PgExecutor, Type};

#[derive(Debug, Clone, Type)]
#[sqlx(transparent)]
pub struct EventData(Value);

#[derive(Debug, Clone, FromRow)]
pub struct DbEvent {
    pub id: Id,
    pub event_type: EventType,
    pub event_data: EventData,
    pub timestamp: DateTime,
}

impl DbEvent {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO events (id, event_type, event_data, event_timestamp)
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
