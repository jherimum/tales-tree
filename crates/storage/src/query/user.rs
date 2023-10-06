use commons::id::Id;
use sqlx::PgExecutor;

use crate::{model::follow::Follow, model::user::User, StorageError};

use super::follow::QueryFollow;

#[async_trait::async_trait]
impl QueryUser for User {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(
            sqlx::query_as("INSERT INTO users (id) VALUES ($1) RETURNING *")
                .bind(self.id())
                .fetch_one(exec)
                .await?,
        )
    }

    async fn find<'e, E: PgExecutor<'e>>(exec: E, id: &Id) -> Result<Option<Self>, StorageError> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(exec)
            .await?)
    }

    async fn is_friend<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
        user_id: Id,
    ) -> Result<bool, StorageError> {
        <Follow as QueryFollow>::follow_each_other(exec, self.id(), &user_id).await
    }
}

#[async_trait::async_trait]
pub trait QueryUser {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<User, StorageError>;

    async fn find<'e, E: PgExecutor<'e>>(exec: E, id: &Id) -> Result<Option<User>, StorageError>;

    async fn is_friend<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
        user_id: Id,
    ) -> Result<bool, StorageError>;
}
