use sqlx::PgExecutor;

use crate::{
    id::Id,
    storage::{like::Like, StorageError},
};

#[async_trait::async_trait]
impl ActiveLike for Like {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO likes (user_id, fragment_id, created_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(self.user_id())
        .bind(self.fragment_id())
        .bind(self.created_at())
        .fetch_one(exec)
        .await?)
    }

    async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError> {
        Ok(sqlx::query(
            r#"
            DELETE FROM likes
            WHERE 
                user_id = $1 AND 
                fragment_id = $2
            "#,
        )
        .bind(self.user_id())
        .bind(self.fragment_id())
        .execute(exec)
        .await
        .map(|r| r.rows_affected() > 0)?)
    }

    async fn find<'e, E: PgExecutor<'e>>(
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

#[async_trait::async_trait]
pub trait ActiveLike {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Like, StorageError>;

    async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError>;

    async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        fragment_id: &Id,
        user_id: &Id,
    ) -> Result<Option<Like>, StorageError>;
}
