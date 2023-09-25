use crate::{
    actor::ActorType,
    commands::{
        create_fragment::CreateFragmentCommand, dislike_fragment::DislikeFragmentCommand,
        follow_user::FollowUserCommand, fork_fragment::ForkFragmentCommand,
        like_fragment::LikeFragmentCommand, publish_fragment::PublishFragmentCommand,
        review_fork::ReviewForkCommand, unfollow_user::UnfollowUserCommand,
        update_fragment::UpdateFragmentCommand, Command, CommandType,
    },
    id::Id,
    storage::Entity,
    DateTime,
};
use ::serde::de::DeserializeOwned;
use derive_builder::Builder;
use derive_getters::Getters;
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, Type};

#[derive(Debug, FromRow, Getters, Builder)]
pub struct Task {
    id: Id,
    command_type: CommandType,
    commnad_data: CommandData,
    actor_type: ActorType,
    actor_id: Option<Id>,
    created_at: DateTime,
    scheduled_at: DateTime,
    completed_at: Option<DateTime>,
}

impl Entity for Task {
    type Id = Id;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[derive(Debug, Type, Clone)]
#[sqlx(transparent)]
pub struct CommandData(Value);

impl<C: Serialize> From<C> for CommandData {
    fn from(value: C) -> Self {
        Self(serde_json::to_value(value).unwrap())
    }
}

impl CommandData {
    pub fn into_command<T: DeserializeOwned>(self) -> T {
        serde_json::from_value(self.0).unwrap()
    }
}

// impl From<Task> for Box<dyn Command> {
//     fn from(value: Task) -> Self {
//         type Type = CommandType;
//         match value.command_type {
//             Type::CreateFragment => {
//                 Box::new(value.commnad_data.into_command::<CreateFragmentCommand>())
//             }
//             Type::FollowUser => Box::new(value.commnad_data.into_command::<FollowUserCommand>()),
//             Type::UnfollowUser => {
//                 Box::new(value.commnad_data.into_command::<UnfollowUserCommand>())
//             }
//             Type::LikeFragment => {
//                 Box::new(value.commnad_data.into_command::<LikeFragmentCommand>())
//             }
//             Type::DislikeFragment => {
//                 Box::new(value.commnad_data.into_command::<DislikeFragmentCommand>())
//             }
//             Type::ForkFragment => {
//                 Box::new(value.commnad_data.into_command::<ForkFragmentCommand>())
//             }
//             Type::PublishFragment => {
//                 Box::new(value.commnad_data.into_command::<PublishFragmentCommand>())
//             }
//             Type::UpdateFragment => {
//                 Box::new(value.commnad_data.into_command::<UpdateFragmentCommand>())
//             }
//             Type::ReviewFork => Box::new(value.commnad_data.into_command::<ReviewForkCommand>()),
//         }
//     }
// }
