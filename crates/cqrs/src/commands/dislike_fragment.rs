use super::{Command, CommandBusError, CommandHandlerContext};
use crate::events::FragmentDislikedEvent;
use chrono::Utc;
use commons::{actor::Actor, commands::CommandType, id::Id};
use storage::{
    active::{fragment::ActiveFragment, like::ActiveLike},
    model::{fragment::Fragment, like::Like},
};
use tap::TapFallible;

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
impl Command for DislikeFragmentCommand {
    type Event = FragmentDislikedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::LikeFragment
    }

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let frag = Fragment::find(ctx.pool(), &self.fragment_id).await?.ok_or(
            DislikeFragmentCommandError::FragmentNotFound(self.fragment_id),
        )?;

        if !frag.is_published() {
            return Err(DislikeFragmentCommandError::FragmentNotPublished(self.fragment_id).into());
        }

        let actual_like = Like::find(ctx.pool(), frag.id(), &ctx.actor().id().unwrap())
            .await
            .tap_err(|e| tracing::error!("Failed to find like: {}", e))?;

        match actual_like {
            Some(l) => match l.delete(ctx.tx().as_mut()).await? {
                true => Ok(Some(FragmentDislikedEvent {
                    fragment_id: self.fragment_id,
                    user_id: ctx.actor().id().unwrap(),
                    timestamp: Utc::now().naive_utc(),
                })),
                false => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}
