use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext};
use crate::events::FragmentPublishedEvent;
use anyhow::Context;
use chrono::Utc;
use commons::{actor::Actor, commands::CommandType, id::Id};
use storage::{
    active::fragment::ActiveFragment,
    model::fragment::{Fragment, FragmentState},
};
use tap::TapFallible;

impl Command for PublishFragmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::PublishFragment
    }
}

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
    type Event = FragmentPublishedEvent;

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = ctx.actor().id().unwrap();

        let fragment = Fragment::find(ctx.pool(), &self.fragment_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find fragment: {e:?}"))?
            .ok_or(PublishFragmentCommandError::FragmentNotFound(
                self.fragment_id,
            ))?;

        if !fragment.is_author(user) {
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
            .map(|f| Some(f.into()))
            .context(format!("Failed to update fragment [{}]", self.fragment_id))?)
    }
}

impl From<Fragment> for FragmentPublishedEvent {
    fn from(value: Fragment) -> Self {
        FragmentPublishedEvent {
            fragment_id: *value.id(),
            user_id: *value.author_id(),
            timestamp: *value.last_modified_at(),
        }
    }
}
