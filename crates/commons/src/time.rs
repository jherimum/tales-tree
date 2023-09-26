use chrono::{NaiveDateTime, Utc};
use mockall::automock;
use serde::{Deserialize, Serialize};

#[automock]
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime {
        DateTime::now()
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct DateTime(NaiveDateTime);

impl DateTime {
    pub fn now() -> Self {
        Self(Utc::now().naive_utc())
    }
}
