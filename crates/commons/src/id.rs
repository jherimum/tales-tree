use mockall::automock;
use std::fmt::Display;

#[automock]
pub trait IdGenerator: Send + Sync {
    fn new_id(&self) -> Id;
}

pub struct DefaultIdGenerator;

impl IdGenerator for DefaultIdGenerator {
    fn new_id(&self) -> Id {
        Id::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Id(uuid::Uuid);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
