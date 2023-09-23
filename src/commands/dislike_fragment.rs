use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    events::FragmentDislikedEvent,
    id::Id,
    storage::{
        active::{fragment::ActiveFragment, like::ActiveLike},
        model::fragment::Fragment,
        model::like::Like,
        model::user::User,
    },
};
use chrono::Utc;
use tap::TapFallible;

impl Command for DislikeFragmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::LikeFragment
    }
}

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
    type Event = FragmentDislikedEvent;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
        let frag = Fragment::find(ctx.pool(), &self.fragment_id).await?.ok_or(
            DislikeFragmentCommandError::FragmentNotFound(self.fragment_id),
        )?;

        if !frag.is_published() {
            return Err(DislikeFragmentCommandError::FragmentNotPublished(self.fragment_id).into());
        }

        let actual_like = Like::find(ctx.pool(), frag.id(), user.id())
            .await
            .tap_err(|e| tracing::error!("Failed to find like: {}", e))?;

        match actual_like {
            Some(l) => match l.delete(ctx.tx().as_mut()).await? {
                true => Ok(Some(FragmentDislikedEvent {
                    fragment_id: self.fragment_id,
                    user_id: *user.id(),
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
