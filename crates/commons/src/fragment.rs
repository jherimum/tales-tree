use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Debug, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Content(String);

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}
