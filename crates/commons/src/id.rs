use mockall::automock;
use std::{fmt::Display, str::FromStr};

#[automock]
pub trait IdGenerator: Send + Sync {
    fn new_id(&self) -> Id;
}

pub struct StdIdGenerator;

impl IdGenerator for StdIdGenerator {
    fn new_id(&self) -> Id {
        Id::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Id(uuid::Uuid);

impl FromStr for Id {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(s)?))
    }
}

impl TryFrom<&str> for Id {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        FromStr::from_str(value)
    }
}

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
