use sqlx::PgExecutor;

use crate::storage::{task::Task, StorageError};

#[async_trait::async_trait]
impl ActiveTask for Task {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Task, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO tasks 
            (id, command_type, command_data, actor_type, actor_id, created_at, scheduled_at) 
            VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 ) RETURNING *"#,
        )
        .bind(self.id())
        .bind(self.command_type())
        .bind(self.commnad_data())
        .bind(self.actor_type())
        .bind(self.actor_id())
        .bind(self.created_at())
        .bind(self.scheduled_at())
        .fetch_one(exec)
        .await?)
    }
}

#[async_trait::async_trait]
pub trait ActiveTask {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Task, StorageError>;
}
