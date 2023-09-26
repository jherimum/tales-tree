use sqlx::Type;

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
