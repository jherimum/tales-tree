use crate::id::Id;
use derive_builder::Builder;
use derive_getters::Getters;
use sqlx::{FromRow, PgExecutor};

use super::{follow::Follow, StorageError};

#[derive(Debug, Builder, Clone, FromRow, Getters)]
#[builder(setter(into))]
pub struct User {
    id: Id,
}

impl User {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(
            sqlx::query_as("INSERT INTO users (id) VALUES ($1) RETURNING *")
                .bind(self.id())
                .fetch_one(exec)
                .await?,
        )
    }

    pub async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        id: &Id,
    ) -> Result<Option<User>, StorageError> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(exec)
            .await?)
    }

    pub async fn is_friend<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
        user_id: impl Into<Id>,
    ) -> Result<bool, StorageError> {
        Follow::follow_each_other(exec, self.id(), &user_id.into()).await
    }
}

impl From<User> for Id {
    fn from(value: User) -> Self {
        value.id
    }
}
