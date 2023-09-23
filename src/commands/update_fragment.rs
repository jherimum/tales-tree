use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    events::FragmentUpdatedEvent,
    id::Id,
    storage::{fragment::Fragment, user::User},
};
use tap::TapFallible;

impl Command for UpdateFragmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::UpdateFragment
    }
}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct UpdateFragmentCommand {
    fragment_id: Id,
    content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateFragmentCommandError {
    #[error("Fragment not found: {0}")]
    FragmentNotFound(Id),

    #[error("{0}")]
    Forbidden(&'static str),

    #[error("{0}")]
    InvalidState(&'static str),
}

#[async_trait::async_trait]
impl CommandHandler for UpdateFragmentCommand {
    type Event = FragmentUpdatedEvent;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
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

        if !fragment.is_editable() {
            return Err(
                UpdateFragmentCommandError::InvalidState("Fragment cannot be edited").into(),
            );
        }

        Ok(fragment
            .set_content(self.content.clone())
            .set_last_modified_at(ctx.clock().now())
            .update(ctx.tx().as_mut())
            .await
            .map(Into::into)
            .tap_err(|e| {
                tracing::error!("Failed to update fragment [{:?}]: {e}", self.fragment_id)
            })?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}

impl From<Fragment> for Option<FragmentUpdatedEvent> {
    fn from(value: Fragment) -> Self {
        Some(FragmentUpdatedEvent {
            user_id: *value.author_id(),
            fragment_id: *value.id(),
            content: value.content().clone(),
            timestamp: *value.last_modified_at(),
        })
    }
}
