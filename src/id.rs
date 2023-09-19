#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Id(uuid::Uuid);
