use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::{id::Id, storage::review::ReviewAction, DateTime};

#[derive(Debug, Clone, Type, PartialEq, Eq)]
#[sqlx(type_name = "event_type", rename_all = "snake_case")]
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

    fn timestamp(&self) -> DateTime;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
pub struct FragmentCreatedEvent {
    fragment_id: Id,
    user_id: Id,
    content: String,
    timestamp: DateTime,
}

impl Event for FragmentCreatedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentCreated
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FragmentDislikedEvent {
    pub fragment_id: Id,
    pub user_id: Id,
    pub timestamp: DateTime,
}

impl Event for FragmentDislikedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentDisliked
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FragmentForkedEvent {
    pub fragment_id: Id,
    pub parent_fragment_id: Id,
    pub user_id: Id,
    pub timestamp: DateTime,
    pub content: String,
}

impl Event for FragmentForkedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentForked
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FragmentPublishedEvent {
    pub fragment_id: Id,
    pub user_id: Id,
    pub timestamp: DateTime,
}

impl Event for FragmentPublishedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentPublished
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FragmentUpdatedEvent {
    pub fragment_id: Id,
    pub timestamp: DateTime,
    pub user_id: Id,
    pub content: String,
}

impl Event for FragmentUpdatedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentUpdated
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FragmentForkReviewedEvent {
    pub fragment_id: Id,
    pub reviewer_id: Id,
    pub timestamp: DateTime,
    pub comment: Option<String>,
    pub action: ReviewAction,
}

impl Event for FragmentForkReviewedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentForkReviewed
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FragmentLikedEvent {
    pub fragment_id: Id,
    pub user_id: Id,
    pub timestamp: DateTime,
}

impl Event for FragmentLikedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentLiked
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserFollowedEvent {
    pub follower_id: Id,
    pub followee_id: Id,
    pub timestamp: DateTime,
}

impl Event for UserFollowedEvent {
    fn event_type(&self) -> EventType {
        EventType::UserFollowed
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserUnfollowedEvent {
    pub follower_id: Id,
    pub followee_id: Id,
    pub timestamp: DateTime,
}

impl Event for UserUnfollowedEvent {
    fn event_type(&self) -> EventType {
        EventType::UserUnfollowed
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
}
