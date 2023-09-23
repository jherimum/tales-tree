use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    events::{FragmentCreatedEvent, FragmentCreatedEventBuilder},
    storage::{
        active::fragment::ActiveFragment,
        fragment::{Fragment, FragmentBuilder, FragmentState},
        user::User,
    },
    Id,
};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
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
            .author_id(*User::try_from(ctx.actor()).unwrap().id())
            .content(self.content.clone())
            .state(FragmentState::Draft)
            .created_at(now)
            .last_modified_at(now)
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
}

impl From<Fragment> for Option<FragmentCreatedEvent> {
    fn from(value: Fragment) -> Self {
        Some(
            FragmentCreatedEventBuilder::default()
                .fragment_id(*value.id())
                .user_id(*value.author_id())
                .content(value.content().clone())
                .timestamp(*value.created_at())
                .build()
                .unwrap(),
        )
    }
}
