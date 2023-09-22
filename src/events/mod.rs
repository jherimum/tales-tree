use serde::Serialize;

use crate::{id::Id, DateTime};

pub enum EventType {
    FragmentCreated,
    FragmentForked,
    FragmentPublished,
    FragmentUpdated,
    FragmentForkReviewed,
    FragmentLiked,
    FragmentDisliked,
    UserFollowed,
    UserUnfollowed,
}

pub trait Event
where
    Self: Serialize + Clone,
{
    fn event_type(&self) -> EventType;
    fn data(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FragmentCreatedEvent {
    pub fragment_id: Id,
    pub user_id: Id,
    pub content: String,
    pub timestamp: DateTime,
}

impl Event for FragmentCreatedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentCreated
    }
}
