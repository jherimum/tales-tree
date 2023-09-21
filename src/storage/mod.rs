pub mod follow;
pub mod fragment;
pub mod like;
pub mod review;

use crate::Id;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub trait Entity {
    fn id(&self) -> Id;

    fn same_as(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
