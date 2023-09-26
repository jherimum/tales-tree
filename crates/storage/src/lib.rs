pub mod active;
pub mod model;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub trait Entity {
    type Id: PartialEq;

    fn id(&self) -> Self::Id;

    fn same_as(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
