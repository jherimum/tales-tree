use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Debug, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Comment(String);
