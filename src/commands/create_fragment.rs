use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    events::FragmentCreatedEvent,
    storage::{
        fragment::{Fragment, FragmentBuilder, FragmentState},
        user::User,
    },
    Id,
};
use chrono::Utc;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateFragmentCommandError {}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct CreateFragmentCommand {
    pub id: Id,
    pub content: String,
}

impl Command for CreateFragmentCommand {}

#[async_trait::async_trait]
impl CommandHandler for CreateFragmentCommand {
    type Event = FragmentCreatedEvent;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        Ok(FragmentBuilder::default()
            .id(self.id)
            .author_id(User::try_from(ctx.actor()).unwrap())
            .content(self.content.clone())
            .state(FragmentState::Draft)
            .created_at(Utc::now().naive_utc())
            .last_modified_at(Utc::now().naive_utc())
            .build()
            .tap_err(|e| tracing::error!("Failed to build fragment: {e}"))
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .map(Into::into)
            .tap_err(|e| tracing::error!("Failed to save fragment:{e}"))?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateFragment
    }
}

impl From<Fragment> for Option<FragmentCreatedEvent> {
    fn from(value: Fragment) -> Self {
        Some(FragmentCreatedEvent {
            fragment_id: *value.id(),
            user_id: *value.author_id(),
            content: value.content().clone(),
            timestamp: *value.created_at(),
        })
    }
}
