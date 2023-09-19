use crate::{DateTime, Id};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor};

use super::{Entity, StorageError};

#[derive(Debug, Builder, Clone, FromRow)]
#[builder(setter(into))]
pub struct Tale {
    id: Id,
    author_id: Id,
    content: String,
    state: TaleState,
    #[builder(default)]
    parent_id: Option<Id>,
    created_at: DateTime,
}

impl Entity for Tale {
    fn id(&self) -> Id {
        self.id
    }
}

impl Tale {
    pub fn root(&self) -> bool {
        self.parent_id.is_none()
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        sqlx::query_as("INSERT INTO tales (id, author_id, content, state, parent_id, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
        .bind(self.id)
        .bind(self.author_id)
        .bind(self.content)
        .bind(self.state)
        .bind(self.parent_id)
        .bind(self.created_at)
        .fetch_one(exec).await
        .map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, Copy)]
#[sqlx(type_name = "tale_state", rename_all = "snake_case")]
pub enum TaleState {
    Draft,
    Published,
    Submitted,
    Rejected,
    ChangesRequested,
}
