use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    events::UserFollowedEvent,
    id::Id,
    storage::{
        active::follow::ActiveFollow,
        follow::{Follow, FollowBuilder},
        user::User,
    },
};
use tap::TapFallible;

impl Command for FollowUserCommand {
    fn command_type(&self) -> CommandType {
        CommandType::FollowUser
    }
}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct FollowUserCommand {
    followee_user_id: Id,
}

#[async_trait::async_trait]
impl CommandHandler for FollowUserCommand {
    type Event = UserFollowedEvent;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
        let actual_follow = Follow::find(ctx.pool(), user.id(), &self.followee_user_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find follow: {e}"))?;

        if actual_follow.is_some() {
            return Ok(None);
        }

        Ok(FollowBuilder::default()
            .follower_id(*user.id())
            .followee_id(self.followee_user_id)
            .build()
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save follow: {e}"))
            .map(Into::into)?)
    }

    fn supports(&self, actor: &crate::actor::Actor) -> bool {
        actor.is_user()
    }
}

impl From<Follow> for Option<UserFollowedEvent> {
    fn from(value: Follow) -> Self {
        Some(UserFollowedEvent {
            follower_id: *value.follower_id(),
            followee_id: *value.followee_id(),
            timestamp: *value.created_at(),
        })
    }
}
