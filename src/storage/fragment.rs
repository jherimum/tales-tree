use super::review::ReviewAction;
use super::user::User;
use super::{Entity, StorageError};
use crate::{DateTime, Id};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct Path(Vec<Id>);

impl Path {
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(&self, id: Id) -> Self {
        let mut new_path = self.0.clone();
        new_path.push(id);
        Self(new_path)
    }
}

impl AsRef<[Id]> for Path {
    fn as_ref(&self) -> &[Id] {
        self.0.as_ref()
    }
}

#[derive(Debug, Builder, Clone, FromRow, Getters, Setters, PartialEq, Eq)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Fragment {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    author_id: Id,

    content: String,

    state: FragmentState,

    #[builder(default)]
    #[setters(skip)]
    parent_id: Option<Id>,

    #[builder(default)]
    #[setters(skip)]
    path: Path,

    #[setters(skip)]
    created_at: DateTime,

    last_modified_at: DateTime,
}

impl Entity for Fragment {
    fn id(&self) -> Id {
        self.id
    }
}

impl Fragment {
    pub fn is_author(&self, author: &User) -> bool {
        self.author_id == *author.id()
    }

    pub fn is_published(&self) -> bool {
        self.state == FragmentState::Published
    }

    pub fn is_waiting_review(&self) -> bool {
        self.state == FragmentState::WaitingReview
    }

    pub fn is_editable(&self) -> bool {
        self.is_draft() || self.is_waiting_changes()
    }

    pub fn is_waiting_changes(&self) -> bool {
        self.state == FragmentState::WaitingChanges
    }

    pub fn is_draft(&self) -> bool {
        self.state == FragmentState::Draft
    }

    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    pub fn is_fork(&self) -> bool {
        self.parent_id.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, Copy)]
#[sqlx(type_name = "fragment_state", rename_all = "snake_case")]
pub enum FragmentState {
    Draft,
    Published,
    WaitingReview,
    Rejected,
    WaitingChanges,
}

impl From<ReviewAction> for FragmentState {
    fn from(value: ReviewAction) -> Self {
        match value {
            ReviewAction::Approve => FragmentState::Published,
            ReviewAction::Reject => FragmentState::Rejected,
            ReviewAction::RequestChanges => FragmentState::WaitingChanges,
        }
    }
}

#[async_trait::async_trait]
impl ActiveFragment for Fragment {
    async fn get_parent<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Option<Self>, StorageError> {
        query_as("SELECT * from fragments WHERE id = $1")
            .bind(self.parent_id)
            .fetch_optional(exec)
            .await
            .map_err(Into::into)
    }

    async fn children<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Self>, StorageError> {
        query_as("SELECT * from fragments WHERE parent_id = $1")
            .bind(self.id)
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
        .bind(self.id)
        .bind(self.author_id)
        .bind(self.content)
        .bind(self.state)
        .bind(self.parent_id)
        .bind(self.created_at)
        .bind(self.last_modified_at)
        .bind(self.path)
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
        .bind(self.id)
        .bind(self.content)
        .bind(self.state)
        .bind(self.last_modified_at)
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
