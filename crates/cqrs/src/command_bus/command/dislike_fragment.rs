use super::Command;
use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::FragmentDislikedEvent;
use commons::{actor::ActorTrait, commands::CommandType, id::Id};
use storage::{
    model::{fragment::Fragment, like::Like},
    query::{fragment::QueryFragment, like::QueryLike},
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
#[builder(setter(into))]
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

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let frag = Fragment::find(ctx.pool(), &self.fragment_id).await?.ok_or(
            DislikeFragmentCommandError::FragmentNotFound(self.fragment_id),
        )?;

        if !frag.is_published() {
            return Err(DislikeFragmentCommandError::FragmentNotPublished(self.fragment_id).into());
        }

        let actual_like = Like::find(ctx.pool(), frag.id(), &ctx.actor().actor().id().unwrap())
            .await
            .tap_err(|e| tracing::error!("Failed to find like: {}", e))?;

        match actual_like {
            Some(l) => match l.delete(ctx.tx().as_mut()).await? {
                true => Ok(Some(FragmentDislikedEvent {
                    fragment_id: self.fragment_id,
                    user_id: ctx.actor().actor().id().unwrap(),
                    timestamp: ctx.clock().now(),
                })),
                false => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn supports<A: ActorTrait>(&self, actor: &A) -> bool {
        actor.actor().is_user()
    }
}
