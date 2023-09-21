use derive_builder::Builder;
use sqlx::{FromRow, PgExecutor};

use crate::{id::Id, DateTime};

use super::StorageError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRow, Builder)]
#[builder(setter(into))]
pub struct Like {
    id: Id,
    user_id: Id,
    fragment_id: Id,
    created_at: DateTime,
}

impl Like {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        todo!()
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError> {
        todo!()
    }

    pub async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        fragment_id: &Id,
        user_id: &Id,
    ) -> Result<Option<Self>, StorageError> {
        todo!()
    }
}
