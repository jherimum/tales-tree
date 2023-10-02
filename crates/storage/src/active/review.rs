use sqlx::PgExecutor;

use crate::{model::review::Review, StorageError};

#[async_trait::async_trait]
impl ActiveReview for Review {
    async fn save<'e, E: PgExecutor<'e>>(self, _: E) -> Result<Self, StorageError> {
        todo!()
    }
}

#[async_trait::async_trait]
pub trait ActiveReview {
    async fn save<'e, E: PgExecutor<'e>>(self, _: E) -> Result<Review, StorageError>;
}
