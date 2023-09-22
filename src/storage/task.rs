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
    DateTime,
};
use ::serde::de::DeserializeOwned;
use derive_builder::Builder;
use derive_getters::Getters;
use serde::Serialize;
use serde_json::Value;
use sqlx::{FromRow, PgExecutor, Type};

use super::StorageError;

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

impl Task {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Task, StorageError> {
        Ok(sqlx::query_as(
            r#"
            INSERT INTO tasks 
            (id, command_type, command_data, actor_type, actor_id, created_at, scheduled_at) 
            VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 ) RETURNING *"#,
        )
        .bind(&self.id)
        .bind(&self.command_type)
        .bind(&self.commnad_data)
        .bind(&self.actor_type)
        .bind(&self.actor_id)
        .bind(&self.created_at)
        .bind(&self.scheduled_at)
        .fetch_one(exec)
        .await?)
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

impl Into<Box<dyn Command>> for Task {
    fn into(self) -> Box<dyn Command> {
        type Type = CommandType;
        match self.command_type {
            Type::CreateFragment => {
                Box::new(self.commnad_data.into_command::<CreateFragmentCommand>())
            }
            Type::FollowUser => Box::new(self.commnad_data.into_command::<FollowUserCommand>()),
            Type::UnfollowUser => Box::new(self.commnad_data.into_command::<UnfollowUserCommand>()),
            Type::LikeFragment => Box::new(self.commnad_data.into_command::<LikeFragmentCommand>()),
            Type::DislikeFragment => {
                Box::new(self.commnad_data.into_command::<DislikeFragmentCommand>())
            }
            Type::ForkFragment => Box::new(self.commnad_data.into_command::<ForkFragmentCommand>()),
            Type::PublishFragment => {
                Box::new(self.commnad_data.into_command::<PublishFragmentCommand>())
            }
            Type::UpdateFragment => {
                Box::new(self.commnad_data.into_command::<UpdateFragmentCommand>())
            }
            Type::ReviewFork => Box::new(self.commnad_data.into_command::<ReviewForkCommand>()),
        }
    }
}
