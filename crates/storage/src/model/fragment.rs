use super::review::ReviewAction;
use crate::Entity;
use commons::{fragment::Content, id::Id, time::DateTime};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct Path(Vec<Id>);

impl Path {
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    pub const fn new() -> Self {
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

    content: Content,

    #[builder(default)]
    state: FragmentState,

    #[builder(default)]
    #[setters(skip)]
    parent_id: Option<Id>,

    #[builder(default)]
    #[setters(skip)]
    path: Path,

    #[sqlx(rename = "_end")]
    #[builder(default)]
    end: bool,

    #[setters(skip)]
    created_at: DateTime,

    last_modified_at: DateTime,
}

impl Entity for Fragment {
    type Id = Id;
    fn id(&self) -> Id {
        self.id
    }
}

impl Fragment {
    pub fn is_author(&self, author: impl Into<Id>) -> bool {
        self.author_id == author.into()
    }

    pub fn is_publishable(&self) -> bool {
        if self.is_fork() {
            return self.is_approved();
        }

        self.is_draft()
    }

    pub fn is_submittable(&self) -> bool {
        self.is_fork() && (self.is_draft() || self.is_waiting_changes())
    }

    pub fn is_published(&self) -> bool {
        self.state == FragmentState::Published
    }

    pub fn is_submitted(&self) -> bool {
        self.state == FragmentState::Submitted
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

    pub const fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    pub const fn is_fork(&self) -> bool {
        self.parent_id.is_some()
    }

    pub fn is_approved(&self) -> bool {
        self.state == FragmentState::Approved
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, Copy, Default)]
#[sqlx(type_name = "fragment_state", rename_all = "snake_case")]
pub enum FragmentState {
    #[default]
    Draft,
    Published,
    Submitted,
    Rejected,
    Approved,
    WaitingChanges,
}

impl From<ReviewAction> for FragmentState {
    fn from(value: ReviewAction) -> Self {
        match value {
            ReviewAction::Approve => Self::Approved,
            ReviewAction::Reject => Self::Rejected,
            ReviewAction::RequestChanges => Self::WaitingChanges,
        }
    }
}
