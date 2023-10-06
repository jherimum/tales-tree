use super::Command;
use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::FragmentLikedEvent;
use commons::{commands::CommandType, id::Id};
use storage::{
    model::{
        fragment::Fragment,
        like::{Like, LikeBuilder},
    },
    query::{fragment::QueryFragment, like::QueryLike},
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
#[builder(setter(into))]
pub struct LikeFragmentCommand {
    fragment_id: Id,
}

#[derive(Debug, thiserror::Error)]
pub enum LikeFragmentCommandError {
    #[error("Fragment {0} not found")]
    FragmentNotFound(Id),

    #[error("Fragment {0} not published")]
    FragmentNotPublished(Id),
}

#[async_trait::async_trait]
impl Command for LikeFragmentCommand {
    type Event = FragmentLikedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::LikeFragment
    }
    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = ctx.actor().id().unwrap();
        let frag = Fragment::find(ctx.pool(), &self.fragment_id)
            .await?
            .ok_or(LikeFragmentCommandError::FragmentNotFound(self.fragment_id))?;

        if !frag.is_published() {
            return Err(LikeFragmentCommandError::FragmentNotPublished(self.fragment_id).into());
        }

        let actual_like = Like::find(ctx.pool(), frag.id(), &user)
            .await
            .tap_err(|e| tracing::error!("Failed to find like: {}", e))?;

        match actual_like {
            Some(_) => Ok(None),
            None => Ok(LikeBuilder::default()
                .fragment_id(*frag.id())
                .user_id(user)
                .created_at(ctx.clock().now())
                .build()
                .tap_err(|e| tracing::error!("Failed to build like: {}", e))
                .map_err(anyhow::Error::from)?
                .save(ctx.tx().as_mut())
                .await
                .tap_err(|e| tracing::error!("Failed to save like: {e}"))
                .map(|l| Some(l.into()))?),
        }
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        actor.actor().is_user()
    }
}

impl From<Like> for FragmentLikedEvent {
    fn from(value: Like) -> Self {
        Self {
            fragment_id: *value.fragment_id(),
            user_id: *value.user_id(),
            timestamp: *value.created_at(),
        }
    }
}
