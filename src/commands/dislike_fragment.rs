use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    id::Id,
    storage::{fragment::Fragment, like::Like, user::User},
};
use tap::TapFallible;

impl Command for DislikeFragmentCommand {}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct DislikeFragmentCommand {
    fragment_id: Id,
}

#[derive(Debug, thiserror::Error)]
pub enum DislikeFragmentCommandError {
    #[error("Fragment {0} not found")]
    FragmentNotFound(Id),

    #[error("Fragment {0} not published")]
    FragmentNotPublished(Id),
}

#[async_trait::async_trait]
impl CommandHandler for DislikeFragmentCommand {
    type Output = bool;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
        let frag = Fragment::find(ctx.pool(), &self.fragment_id).await?.ok_or(
            DislikeFragmentCommandError::FragmentNotFound(self.fragment_id),
        )?;

        if !frag.is_published() {
            return Err(DislikeFragmentCommandError::FragmentNotPublished(self.fragment_id).into());
        }

        let actual_like = Like::find(ctx.pool(), &frag.id(), &user.id())
            .await
            .tap_err(|e| tracing::error!("Failed to find like: {}", e))?;

        match actual_like {
            Some(l) => Ok(l.delete(ctx.tx().as_mut()).await?),
            None => Ok(false),
        }
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }

    fn command_type(&self) -> CommandType {
        CommandType::LikeFragment
    }
}
