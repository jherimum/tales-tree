use super::Command;
use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::UserFollowedEvent;
use commons::{commands::CommandType, id::Id};
use storage::{
    active::follow::ActiveFollow,
    model::follow::{Follow, FollowBuilder},
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
#[builder(setter(into))]
pub struct FollowUserCommand {
    following_user_id: Id,
}

#[async_trait::async_trait]
impl Command for FollowUserCommand {
    type Event = UserFollowedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::FollowUser
    }

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = ctx.actor().id().unwrap();
        let actual_follow = Follow::find(ctx.pool(), &user, &self.following_user_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find follow: {e}"))?;

        if actual_follow.is_some() {
            return Ok(None);
        }

        Ok(FollowBuilder::default()
            .follower_id(user)
            .following_id(self.following_user_id)
            .build()
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save follow: {e}"))
            .map(|f| Some(f.into()))?)
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        actor.actor().is_user()
    }
}

impl From<Follow> for UserFollowedEvent {
    fn from(value: Follow) -> Self {
        UserFollowedEvent {
            follower_id: *value.follower_id(),
            following_id: *value.following_id(),
            timestamp: *value.created_at(),
        }
    }
}
