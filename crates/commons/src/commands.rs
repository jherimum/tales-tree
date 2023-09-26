use sqlx::Type;

#[derive(Debug, Type, Clone)]
pub enum CommandType {
    CreateFragment,
    FollowUser,
    UnfollowUser,
    LikeFragment,
    DislikeFragment,
    ForkFragment,
    PublishFragment,
    UpdateFragment,
    ReviewFork,
}
