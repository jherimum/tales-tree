use sqlx::PgExecutor;

use crate::{
    id::Id,
    storage::{model::follow::Follow, StorageError},
};

#[async_trait::async_trait]
impl ActiveFollow for Follow {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO follows (follower_id, followee_id, created_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(self.follower_id())
        .bind(self.followee_id())
        .bind(self.created_at())
        .fetch_one(exec)
        .await?)
    }

    async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError> {
        Ok(sqlx::query(
            r#"
            DELETE FROM follows
            WHERE follower_id = $1 AND followee_id = $2
            "#,
        )
        .bind(self.follower_id())
        .bind(self.followee_id())
        .execute(exec)
        .await
        .map(|r| r.rows_affected() > 0)?)
    }

    async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        follower_id: &Id,
        followee_id: &Id,
    ) -> Result<Option<Self>, StorageError> {
        Ok(sqlx::query_as(
            r#"
                    SELECT * 
                    FROM follows 
                    WHERE 
                        follower_id = $1 AND 
                        followee_id = $2"#,
        )
        .bind(follower_id)
        .bind(followee_id)
        .fetch_optional(exec)
        .await?)
    }

    async fn follow_each_other<'e, E: PgExecutor<'e>>(
        exec: E,
        follower_id: &Id,
        followee_id: &Id,
    ) -> Result<bool, StorageError> {
        Ok(sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(1)
            FROM follows f
            WHERE 
                (f.follower_id = $1 AND f.followee_id = $2) 
                OR 
                (f.follower_id = $2 AND f.followee_id = $1)
            "#,
        )
        .bind(follower_id)
        .bind(followee_id)
        .fetch_one(exec)
        .await
        .map(|c| c > 1)?)
    }
}

#[async_trait::async_trait]
pub trait ActiveFollow: Send {
    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Follow, StorageError>;

    async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError>;

    async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        follower_id: &Id,
        followee_id: &Id,
    ) -> Result<Option<Follow>, StorageError>;

    async fn follow_each_other<'e, E: PgExecutor<'e>>(
        exec: E,
        follower_id: &Id,
        followee_id: &Id,
    ) -> Result<bool, StorageError>;
}
