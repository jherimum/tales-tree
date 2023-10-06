use super::Command;
use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::{FragmentCreatedEvent, FragmentCreatedEventBuilder};
use commons::fragment::Content;
use commons::{actor::ActorTrait, commands::CommandType, id::Id};
use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use storage::{
    model::fragment::{Fragment, FragmentBuilder},
    query::fragment::QueryFragment,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateFragmentCommandError {}

#[derive(Debug, Builder, Deserialize, Serialize, Getters)]
#[builder(setter(into))]
pub struct CreateFragmentCommand {
    fragment_id: Id,
    content: Content,
}

#[async_trait::async_trait]
impl Command for CreateFragmentCommand {
    type Event = FragmentCreatedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::CreateFragment
    }
    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let now = ctx.clock().now();
        Ok(FragmentBuilder::default()
            .id(self.fragment_id)
            .author_id(ctx.actor().actor().id().unwrap())
            .content(self.content.clone())
            .created_at(now)
            .last_modified_at(now)
            .build()
            .tap_err(|e| tracing::error!("Failed to build fragment: {e}"))
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .map(Into::into)
            .map(Some)
            .tap_err(|e| tracing::error!("Failed to save fragment:{e}"))?)
    }

    fn supports<A: ActorTrait>(&self, actor: &A) -> bool {
        actor.actor().is_user()
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
