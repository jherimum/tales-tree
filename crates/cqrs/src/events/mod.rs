use commons::{
    actor::Actor, events::EventType, fragment::Content, id::Id, review::Comment, time::DateTime,
};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use storage::model::review::ReviewAction;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct ForkSubmittedEvent {
    pub fragment_id: Id,
    pub timestamp: DateTime,
    pub actor: Actor,
}

impl Event for ForkSubmittedEvent {
    fn event_type(&self) -> EventType {
        EventType::ForkSubmitted
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        self.actor
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct FragmentCreatedEvent {
    fragment_id: Id,
    user_id: Id,
    content: Content,
    end: bool,
    timestamp: DateTime,
}

impl Event for FragmentCreatedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentCreated
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        Actor::User(self.user_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct FragmentDislikedEvent {
    pub fragment_id: Id,
    pub timestamp: DateTime,
    pub user_id: Id,
}

impl Event for FragmentDislikedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentDisliked
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        Actor::User(self.user_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct FragmentForkedEvent {
    pub fragment_id: Id,
    pub user_id: Id,
    pub content: Content,
    pub end: bool,
    pub parent_fragment_id: Id,
    pub timestamp: DateTime,
}

impl Event for FragmentForkedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentForked
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        Actor::User(self.user_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct FragmentPublishedEvent {
    pub fragment_id: Id,
    pub timestamp: DateTime,
    pub actor: Actor,
}

impl Event for FragmentPublishedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentPublished
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        self.actor
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct FragmentUpdatedEvent {
    pub fragment_id: Id,
    pub timestamp: DateTime,
    pub content: Content,
    pub end: bool,
    pub actor: Actor,
}

impl Event for FragmentUpdatedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentUpdated
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        self.actor
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct FragmentForkReviewedEvent {
    pub fragment_id: Id,
    pub timestamp: DateTime,
    pub comment: Option<Comment>,
    pub action: ReviewAction,
    pub actor: Actor,
}

impl Event for FragmentForkReviewedEvent {
    fn event_type(&self) -> EventType {
        EventType::FragmentForkReviewed
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        self.actor
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
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
    fn actor(&self) -> Actor {
        Actor::User(self.user_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct UserFollowedEvent {
    pub follower_id: Id,
    pub following_id: Id,
    pub timestamp: DateTime,
}

impl Event for UserFollowedEvent {
    fn event_type(&self) -> EventType {
        EventType::UserFollowed
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        Actor::User(self.following_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Builder, Getters)]
#[builder(setter(into))]
pub struct UserUnfollowedEvent {
    pub follower_id: Id,
    pub following_id: Id,
    pub timestamp: DateTime,
}

impl Event for UserUnfollowedEvent {
    fn event_type(&self) -> EventType {
        EventType::UserUnfollowed
    }
    fn timestamp(&self) -> DateTime {
        self.timestamp
    }
    fn actor(&self) -> Actor {
        Actor::User(self.following_id)
    }
}

pub trait Event: Send + Sync + Debug {
    fn event_type(&self) -> EventType;
    fn data(&self) -> &Self {
        self
    }

    fn timestamp(&self) -> DateTime;

    fn actor(&self) -> Actor;
}
