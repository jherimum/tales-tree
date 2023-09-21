use super::{CommandBusError, CommandHandler, CommandHandlerContext};
use crate::{
    id::Id,
    storage::{
        follow::{Follow, FollowBuilder},
        user::User,
    },
};
use tap::TapFallible;

#[derive(Clone, Copy)]
pub enum FollowAction {
    Follow,
    Unfollow,
}

pub struct FollowUserCommand {
    followee_user_id: Id,
    action: FollowAction,
}

#[async_trait::async_trait]
impl CommandHandler for FollowUserCommand {
    type Output = bool;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
        let actual_follow = Follow::find(ctx.pool(), user.id(), &self.followee_user_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find follow: {e}"))?;

        Ok(match (self.action, actual_follow) {
            (FollowAction::Follow, None) => FollowBuilder::default()
                .follower_id(*user.id())
                .followee_id(self.followee_user_id)
                .build()
                .map_err(anyhow::Error::from)?
                .save(ctx.tx().as_mut())
                .await
                .tap_err(|e| tracing::error!("Failed to save follow: {e}"))
                .map(|_| true)?,
            (FollowAction::Unfollow, Some(f)) => f
                .delete(ctx.tx().as_mut())
                .await
                .tap_err(|e| tracing::error!("Failed to delete follow:{e}"))?,
            _ => true,
        })
    }

    fn supports(&self, actor: &crate::actor::Actor) -> bool {
        actor.is_user()
    }
}
