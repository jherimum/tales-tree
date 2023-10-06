use sqlx::PgExecutor;

use crate::{model::review::Review, StorageError};

#[async_trait::async_trait]
impl QueryReview for Review {
    async fn save<'e, E: PgExecutor<'e>>(self, _: E) -> Result<Self, StorageError> {
        todo!()
    }
}

#[async_trait::async_trait]
pub trait QueryReview {
    async fn save<'e, E: PgExecutor<'e>>(self, _: E) -> Result<Review, StorageError>;
}
