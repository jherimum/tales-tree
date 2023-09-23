use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    events::FragmentUpdatedEvent,
    id::Id,
    storage::{active::fragment::ActiveFragment, fragment::Fragment, user::User},
};
use derive_getters::Getters;
use tap::TapFallible;

impl Command for UpdateFragmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::UpdateFragment
    }
}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize, Getters)]
#[builder(setter(into))]
pub struct UpdateFragmentCommand {
    fragment_id: Id,
    content: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum UpdateFragmentCommandError {
    #[error("Fragment not found: {0}")]
    FragmentNotFound(Id),

    #[error("{0}")]
    UserWithoutPermission(Id),

    #[error("Fragment {0} is not editable")]
    NonEditableFragment(Id),
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
            return Err(UpdateFragmentCommandError::UserWithoutPermission(*user.id()).into());
        }

        if !fragment.is_editable() {
            return Err(UpdateFragmentCommandError::NonEditableFragment(self.fragment_id).into());
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
