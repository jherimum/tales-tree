use crate::DateTime;
use mockall::automock;

#[automock]
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime {
        chrono::Utc::now().naive_utc()
    }
}
