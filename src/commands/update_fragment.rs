use super::{CommandBusError, CommandHandler, CommandHandlerContext};
use crate::{
    actor::Actor,
    id::Id,
    storage::{fragment::Fragment, StorageError},
    User,
};
use chrono::Utc;
use tap::TapFallible;

pub struct UpdateFragmentCommand {
    fragment_id: Id,
    content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateFragmentCommandError {
    #[error("Fragment not found: {0}")]
    FragmentNotFound(Id),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error("{0}")]
    Forbidden(&'static str),

    #[error("{0}")]
    InvalidState(&'static str),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait::async_trait]
impl CommandHandler for UpdateFragmentCommand {
    type Output = Fragment;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let user = User::try_from(ctx.actor())?;

        let fragment = Fragment::find(ctx.pool(), &self.fragment_id).await?.ok_or(
            UpdateFragmentCommandError::FragmentNotFound(self.fragment_id),
        )?;

        if !fragment.is_author(&user) {
            return Err(UpdateFragmentCommandError::Forbidden(
                "Only the author can update a fragment",
            )
            .into());
        }

        if !fragment.is_draft() {
            return Err(UpdateFragmentCommandError::InvalidState("Fragment is not a draft").into());
        }

        Ok(fragment
            .set_content(self.content.clone())
            .set_last_modified_at(Utc::now().naive_utc())
            .update(ctx.tx().as_mut())
            .await
            .tap_err(|e| {
                tracing::error!("Failed to update fragment [{:?}]: {e}", self.fragment_id)
            })?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}
