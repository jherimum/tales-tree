use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    id::Id,
    storage::{
        fragment::{Fragment, FragmentState},
        user::User,
    },
};
use anyhow::Context;
use chrono::Utc;
use tap::TapFallible;

impl Command for PublishFragmentCommand {}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct PublishFragmentCommand {
    pub fragment_id: Id,
}

#[derive(Debug, thiserror::Error)]
pub enum PublishFragmentCommandError {
    #[error("Fragment not found: {0}")]
    FragmentNotFound(Id),

    #[error("{0}")]
    InvalidState(&'static str),

    #[error("{0}")]
    Forbidden(&'static str),
}

#[async_trait::async_trait]
impl CommandHandler for PublishFragmentCommand {
    type Output = Fragment;

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        let user_actor = User::try_from(ctx.actor())?;

        let fragment = Fragment::find(ctx.pool(), &self.fragment_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find fragment: {e:?}"))?
            .ok_or(PublishFragmentCommandError::FragmentNotFound(
                self.fragment_id,
            ))?;

        if !fragment.is_author(&user_actor) {
            return Err(PublishFragmentCommandError::Forbidden(
                "Only the fragment author can publish it",
            )
            .into());
        }

        if !fragment.is_draft() {
            return Err(PublishFragmentCommandError::InvalidState(
                "fragment should be in draft state to be published",
            )
            .into());
        }

        let state = if fragment.is_fork() {
            FragmentState::WaitingReview
        } else {
            FragmentState::Published
        };

        Ok(fragment
            .set_state(state)
            .set_last_modified_at(Utc::now().naive_utc())
            .update(ctx.tx().as_mut())
            .await
            .context(format!("Failed to update fragment [{}]", self.fragment_id))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::PublishFragment
    }
}
