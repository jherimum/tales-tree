use super::Command;
use crate::{
    command_bus::{bus::Ctx, error::CommandBusError},
    events::ForkSubmittedEvent,
};
use commons::{
    actor::{Actor, ActorTrait, ActorType},
    commands::CommandType,
    id::Id,
};
use storage::{
    model::fragment::{Fragment, FragmentState},
    query::fragment::QueryFragment,
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
#[builder(setter(into))]
pub struct SubmitForkCommand {
    pub fragment_id: Id,
}

#[derive(Debug, thiserror::Error)]
pub enum SubmitForkCommandError {
    #[error("Fork not found: {0}")]
    ForkNotFound(Id),
    #[error("{0}")]
    Forbidden(&'static str),

    #[error("{0}")]
    InvalidState(&'static str),
}

#[async_trait::async_trait]
impl Command for SubmitForkCommand {
    type Event = ForkSubmittedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::SubmitFork
    }

    fn supports<A: ActorTrait>(&self, actor: &A) -> bool {
        ActorType::User == actor.actor_type()
    }

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let fragment = Fragment::find(ctx.pool(), &self.fragment_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find fork: {e:?}"))?
            .ok_or(SubmitForkCommandError::ForkNotFound(self.fragment_id))?;

        if !fragment.is_author(ctx.actor().id().unwrap()) {
            return Err(
                SubmitForkCommandError::Forbidden("Only the fork author can submit it").into(),
            );
        }

        if !fragment.is_submittable() {
            return Err(SubmitForkCommandError::InvalidState("Fork can not be submitted").into());
        }

        Ok(fragment
            .set_state(FragmentState::Submitted)
            .set_last_modified_at(ctx.clock().now())
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save fork: {e:?}"))
            .map(Into::into)
            .map(Some)?)
    }
}

impl From<Fragment> for ForkSubmittedEvent {
    fn from(value: Fragment) -> Self {
        Self {
            fragment_id: *value.id(),
            timestamp: *value.last_modified_at(),
            actor: Actor::User(*value.author_id()),
        }
    }
}
