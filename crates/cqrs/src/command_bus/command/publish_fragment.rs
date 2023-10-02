use super::Command;
use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::FragmentPublishedEvent;
use anyhow::Context;
use commons::actor::ActorType;
use commons::{commands::CommandType, id::Id};
use storage::{
    active::fragment::ActiveFragment,
    model::fragment::{Fragment, FragmentState},
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
#[builder(setter(into))]
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
impl Command for PublishFragmentCommand {
    type Event = FragmentPublishedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::PublishFragment
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        ActorType::User == actor.actor_type()
    }

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
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

        if fragment.is_publishable() {
            return Ok(fragment
                .set_state(FragmentState::Published)
                .set_last_modified_at(ctx.clock().now())
                .update(ctx.tx().as_mut())
                .await
                .map(|f| Some(f.into()))
                .context(format!("Failed to update fragment [{}]", self.fragment_id))?);
        }

        Err(PublishFragmentCommandError::InvalidState("fragment is not publishable").into())
    }
}

impl From<Fragment> for FragmentPublishedEvent {
    fn from(value: Fragment) -> Self {
        Self {
            fragment_id: *value.id(),
            timestamp: *value.last_modified_at(),
            actor: commons::actor::Actor::User(*value.author_id()),
        }
    }
}
