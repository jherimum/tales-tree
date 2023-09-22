use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    id::Id,
    storage::{follow::Follow, user::User},
};
use tap::TapFallible;

impl Command for UnfollowUserCommand {}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct UnfollowUserCommand {
    followee_user_id: Id,
}

#[async_trait::async_trait]
impl CommandHandler for UnfollowUserCommand {
    type Output = bool;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
        let actual_follow = Follow::find(ctx.pool(), user.id(), &self.followee_user_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find follow: {e}"))?;

        match actual_follow {
            Some(f) => Ok(f.delete(ctx.tx().as_mut()).await?),
            None => Ok(false),
        }
    }

    fn supports(&self, actor: &crate::actor::Actor) -> bool {
        actor.is_user()
    }

    fn command_type(&self) -> CommandType {
        CommandType::UnfollowUser
    }
}
