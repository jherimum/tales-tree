use super::StorageError;
use crate::{id::Id, DateTime};
use derive_builder::Builder;
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRow, Builder)]
#[builder(setter(into))]
pub struct Follow {
    follower_id: Id,
    followee_id: Id,
    created_at: DateTime,
}

impl Follow {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO follows (follower_id, followee_id, created_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(self.follower_id)
        .bind(self.followee_id)
        .bind(self.created_at)
        .fetch_one(exec)
        .await?)
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError> {
        Ok(sqlx::query(
            r#"
            DELETE FROM follows
            WHERE follower_id = $1 AND followee_id = $2
            "#,
        )
        .bind(self.follower_id)
        .bind(self.followee_id)
        .execute(exec)
        .await
        .map(|r| r.rows_affected() > 0)?)
    }

    pub async fn find<'e, E: PgExecutor<'e>>(
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

    pub async fn follow_each_other<'e, E: PgExecutor<'e>>(
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
