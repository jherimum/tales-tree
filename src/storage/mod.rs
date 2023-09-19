use crate::Id;

pub mod tale;

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
