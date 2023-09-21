use super::StorageError;
use crate::{id::Id, DateTime};
use derive_builder::Builder;
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRow, Builder)]
#[builder(setter(into))]
pub struct Like {
    user_id: Id,
    fragment_id: Id,
    created_at: DateTime,
}

impl Like {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO likes (user_id, fragment_id, created_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(self.user_id)
        .bind(self.fragment_id)
        .bind(self.created_at)
        .fetch_one(exec)
        .await?)
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError> {
        Ok(sqlx::query(
            r#"
            DELETE FROM likes
            WHERE 
                user_id = $1 AND 
                fragment_id = $2
            "#,
        )
        .bind(self.user_id)
        .bind(self.fragment_id)
        .execute(exec)
        .await
        .map(|r| r.rows_affected() > 0)?)
    }

    pub async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        fragment_id: &Id,
        user_id: &Id,
    ) -> Result<Option<Self>, StorageError> {
        sqlx::query_as(
            r#"
                SELECT * 
                FROM likes 
                WHERE fragment_id = $1 AND user_id = $2
            "#,
        )
        .bind(fragment_id)
        .bind(user_id)
        .fetch_optional(exec)
        .await
        .map_err(|e| e.into())
    }
}
