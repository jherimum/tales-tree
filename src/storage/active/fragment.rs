use sqlx::{query_as, PgExecutor};

use crate::{
    id::Id,
    storage::{fragment::Fragment, StorageError},
};

#[async_trait::async_trait]
impl ActiveFragment for Fragment {
    async fn get_parent<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Option<Self>, StorageError> {
        query_as("SELECT * from fragments WHERE id = $1")
            .bind(self.parent_id())
            .fetch_optional(exec)
            .await
            .map_err(Into::into)
    }

    async fn children<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Self>, StorageError> {
        query_as("SELECT * from fragments WHERE parent_id = $1")
            .bind(self.id())
            .fetch_all(exec)
            .await
            .map_err(Into::into)
    }

    async fn find<'e, E: PgExecutor<'e>>(exec: E, id: &Id) -> Result<Option<Self>, StorageError> {
        query_as("SELECT * from fragments  WHERE id = $1")
            .bind(id)
            .fetch_optional(exec)
            .await
            .map_err(Into::into)
    }

    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        query_as(r#"
            INSERT INTO fragments (id, author_id, content, state, parent_id, created_at, last_modified_at, path) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#)
        .bind(self.id())
        .bind(self.author_id())
        .bind(self.content())
        .bind(self.state())
        .bind(self.parent_id())
        .bind(self.created_at())
        .bind(self.last_modified_at())
        .bind(self.path())
        .fetch_one(exec).await
        .map_err(Into::into)
    }

    async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        query_as(
            r#"
            UPDATE fragments 
            SET 
                content = $2, 
                state = $3, 
                last_modified_at = $4
            WHERE id = $1 RETURNING *"#,
        )
        .bind(self.id())
        .bind(self.content())
        .bind(self.state())
        .bind(self.last_modified_at())
        .fetch_one(exec)
        .await
        .map_err(Into::into)
    }
}

#[async_trait::async_trait]
pub trait ActiveFragment {
    async fn get_parent<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Option<Fragment>, StorageError>;

    async fn children<'e, E: PgExecutor<'e>>(&self, exec: E)
        -> Result<Vec<Fragment>, StorageError>;

    async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        id: &Id,
    ) -> Result<Option<Fragment>, StorageError>;

    async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Fragment, StorageError>;

    async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Fragment, StorageError>;
}
