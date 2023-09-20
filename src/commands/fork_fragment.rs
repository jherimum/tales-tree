use super::{CommandBusError, CommandHandler, CommandHandlerContext};
use crate::{
    actor::Actor,
    id::Id,
    storage::fragment::{Fragment, FragmentBuilder, FragmentState},
    User,
};
use chrono::Utc;
use tap::TapFallible;

pub struct ForkFragmentCommand {
    fragment_id: Id,
    parent_fragment_id: Id,
    content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ForkFragmentCommandError {
    #[error("Fragment not found: {0}")]
    FragmentNotFound(Id),

    #[error("Parent fragment not found: {0}")]
    ParentFragmentNotFound(Id),

    #[error("{0}")]
    Forbidden(&'static str),

    #[error("{0}")]
    InvalidState(&'static str),
}

#[async_trait::async_trait]
impl CommandHandler for ForkFragmentCommand {
    type Output = Fragment;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let author = User::try_from(ctx.actor())?;
        let parent_frag = Fragment::find(ctx.pool(), &self.parent_fragment_id)
            .await
            .tap_err(|e| {
                tracing::error!("Failed to find fragment [{}]: {e}", self.parent_fragment_id)
            })?
            .ok_or(ForkFragmentCommandError::ParentFragmentNotFound(
                self.fragment_id,
            ))?;

        if !parent_frag.is_published() {
            return Err(
                ForkFragmentCommandError::InvalidState("Parent fragment is not published").into(),
            );
        }

        if parent_frag.is_author(&author) {
            return Err(
                ForkFragmentCommandError::Forbidden("Cannot fork your own fragment").into(),
            );
        }

        Ok(FragmentBuilder::default()
            .id(self.fragment_id)
            .author_id(author)
            .content(self.content.clone())
            .parent_id(Some(self.parent_fragment_id))
            .state(FragmentState::Draft)
            .created_at(Utc::now().naive_utc())
            .last_modified_at(Utc::now().naive_utc())
            .path(parent_frag.path().append(self.parent_fragment_id.clone()))
            .build()
            .tap_err(|e| tracing::error!("Failed to build fragment: {e}"))
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save fragment: {e}"))?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}
