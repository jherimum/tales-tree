use crate::command_bus::{bus::Command, bus::CommandHandlerContext, error::CommandBusError};
use crate::events::{FragmentCreatedEvent, FragmentCreatedEventBuilder};
use commons::{
    actor::ActorTrait,
    commands::CommandType,
    id::{Id, IdGenerator},
    time::Clock,
};
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
    end: bool,
}

#[async_trait::async_trait]
impl Command for CreateFragmentCommand {
    type Event = FragmentCreatedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::CreateFragment
    }
    async fn handle<A, CL, I>(
        &self,
        ctx: &mut CommandHandlerContext<A, CL, I>,
    ) -> Result<Option<Self::Event>, CommandBusError>
    where
        A: ActorTrait,
        CL: Clock,
        I: IdGenerator,
    {
        let now = ctx.clock().now();
        Ok(FragmentBuilder::default()
            .id(self.fragment_id)
            .author_id(ctx.actor().id().unwrap())
            .content(self.content.clone())
            .state(FragmentState::Draft)
            .end(self.end)
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

    fn supports<A: ActorTrait>(&self, actor: &A) -> bool {
        true
    }
}

impl From<Fragment> for FragmentCreatedEvent {
    fn from(value: Fragment) -> Self {
        FragmentCreatedEventBuilder::default()
            .fragment_id(*value.id())
            .user_id(*value.author_id())
            .content(value.content().clone())
            .timestamp(*value.created_at())
            .end(*value.end())
            .build()
            .unwrap()
    }
}
