use crate::command_bus::bus::Ctx;
use crate::command_bus::{bus::Command, error::CommandBusError};
use crate::events::UserFollowedEvent;
use commons::{commands::CommandType, id::Id};
use storage::{
    active::follow::ActiveFollow,
    model::follow::{Follow, FollowBuilder},
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct FollowUserCommand {
    followee_user_id: Id,
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
        let actual_follow = Follow::find(ctx.pool(), &user, &self.followee_user_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find follow: {e}"))?;

        if actual_follow.is_some() {
            return Ok(None);
        }

        Ok(FollowBuilder::default()
            .follower_id(user)
            .followee_id(self.followee_user_id)
            .build()
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save follow: {e}"))
            .map(|f| Some(f.into()))?)
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        //actor.is_user()
        todo!()
    }
}

impl From<Follow> for UserFollowedEvent {
    fn from(value: Follow) -> Self {
        UserFollowedEvent {
            follower_id: *value.follower_id(),
            followee_id: *value.followee_id(),
            timestamp: *value.created_at(),
        }
    }
}