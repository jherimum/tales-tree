use super::Command;
use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::UserUnfollowedEvent;
use commons::{commands::CommandType, id::Id};
use storage::{active::follow::ActiveFollow, model::follow::Follow};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
#[builder(setter(into))]
pub struct UnfollowUserCommand {
    following_user_id: Id,
}

#[async_trait::async_trait]
impl Command for UnfollowUserCommand {
    type Event = UserUnfollowedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::UnfollowUser
    }

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = ctx.actor().id().unwrap();
        let actual_follow = Follow::find(ctx.pool(), &user, &self.following_user_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find follow: {e}"))?;

        match actual_follow {
            Some(f) => match f.delete(ctx.tx().as_mut()).await? {
                true => Ok(Some(UserUnfollowedEvent {
                    follower_id: user,
                    following_id: self.following_user_id,
                    timestamp: *f.created_at(),
                })),
                false => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        actor.actor().is_user()
    }
}
