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

#[derive(Clone, Debug, PartialEq, Eq, Type, Serialize, Deserialize, Default, Copy)]
pub enum End {
    Yes,
    #[default]
    No,
}

impl End {
    pub fn yes(&self) -> bool {
        End::Yes == *self
    }

    pub fn no(&self) -> bool {
        End::No == *self
    }
}

impl From<bool> for End {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}
