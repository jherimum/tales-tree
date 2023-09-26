use sqlx::Type;

#[derive(Debug, Type, Clone)]
#[sqlx(type_name = "command_type", rename_all = "snake_case")]
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
