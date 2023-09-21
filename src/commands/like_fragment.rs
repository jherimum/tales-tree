use super::{CommandBusError, CommandHandler, CommandHandlerContext};
use crate::{
    actor::Actor,
    id::Id,
    storage::{
        fragment::Fragment,
        like::{Like, LikeBuilder},
    },
    User,
};
use chrono::Utc;
use tap::TapFallible;

#[derive(Debug, Clone, Copy)]
pub enum LikeAction {
    Like,
    Dislike,
}

#[derive(Debug)]
pub struct LikeOrDislikeFragmentCommand {
    fragment_id: Id,
    action: LikeAction,
}

#[derive(Debug, thiserror::Error)]
pub enum LikeOrDislikeFragmentCommandError {
    #[error("Fragment {0} not found")]
    FragmentNotFound(Id),

    #[error("Fragment {0} not published")]
    FragmentNotPublished(Id),
}

#[async_trait::async_trait]
impl CommandHandler for LikeOrDislikeFragmentCommand {
    type Output = bool;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
        let frag = Fragment::find(ctx.pool(), &self.fragment_id).await?.ok_or(
            LikeOrDislikeFragmentCommandError::FragmentNotFound(self.fragment_id),
        )?;

        if !frag.is_published() {
            return Err(
                LikeOrDislikeFragmentCommandError::FragmentNotPublished(self.fragment_id).into(),
            );
        }

        let actual_like = Like::find(ctx.pool(), &frag.id(), &user.id())
            .await
            .tap_err(|e| tracing::error!("Failed to find like: {}", e))?;

        Ok(match (self.action, actual_like) {
            (LikeAction::Like, None) => LikeBuilder::default()
                .id(Id::new())
                .fragment_id(*frag.id())
                .user_id(user)
                .created_at(Utc::now().naive_utc())
                .build()
                .tap_err(|e| tracing::error!("Failed to build like: {}", e))
                .map_err(anyhow::Error::from)?
                .save(ctx.tx.as_mut())
                .await
                .tap_err(|e| tracing::error!("Failed to save like: {e}"))
                .map(|_| true)?,
            (LikeAction::Dislike, Some(like)) => like
                .delete(ctx.tx.as_mut())
                .await
                .tap_err(|e| tracing::error!("Failed to delete like: {e}"))?,
            _ => false,
        })
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}
