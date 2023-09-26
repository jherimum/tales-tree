use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext};
use crate::events::{FragmentCreatedEvent, FragmentCreatedEventBuilder};
use commons::{actor::Actor, commands::CommandType, id::Id};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use storage::{
    active::fragment::ActiveFragment,
    model::fragment::{Fragment, FragmentBuilder, FragmentState},
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateFragmentCommandError {}

#[derive(Debug, Builder, Deserialize, Serialize, Getters)]
pub struct CreateFragmentCommand {
    fragment_id: Id,
    content: String,
}

impl Command for CreateFragmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::CreateFragment
    }
}

#[async_trait::async_trait]
impl CommandHandler for CreateFragmentCommand {
    type Event = FragmentCreatedEvent;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let now = ctx.clock().now();
        Ok(FragmentBuilder::default()
            .id(self.fragment_id)
            .author_id(ctx.actor().id().unwrap())
            .content(self.content.clone())
            .state(FragmentState::Draft)
            .created_at(now)
            .last_modified_at(now)
            .build()
            .tap_err(|e| tracing::error!("Failed to build fragment: {e}"))
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .map(|f| Some(f.into()))
            .tap_err(|e| tracing::error!("Failed to save fragment:{e}"))?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}

impl From<Fragment> for FragmentCreatedEvent {
    fn from(value: Fragment) -> Self {
        FragmentCreatedEventBuilder::default()
            .fragment_id(*value.id())
            .user_id(*value.author_id())
            .content(value.content().clone())
            .timestamp(*value.created_at())
            .build()
            .unwrap()
    }
}
