use sqlx::PgExecutor;

use crate::{model::event::DbEvent, StorageError};

#[async_trait::async_trait]
impl QueryEvent for DbEvent {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO events (id, event_type, event_data, timestamp)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(self.id())
        .bind(self.event_type())
        .bind(self.event_data())
        .bind(self.timestamp())
        .fetch_one(exec)
        .await?)
    }

    async fn all<'e, E: PgExecutor<'e>>(exec: E) -> Result<Vec<Self>, StorageError> {
        Ok(sqlx::query_as(
            r#"
            SELECT * FROM events
            "#,
        )
        .fetch_all(exec)
        .await?)
    }
}

#[async_trait::async_trait]
pub trait QueryEvent {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<DbEvent, StorageError>;

    async fn all<'e, E: PgExecutor<'e>>(exec: E) -> Result<Vec<DbEvent>, StorageError>;
}
