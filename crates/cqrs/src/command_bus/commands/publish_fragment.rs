use crate::command_bus::bus::Ctx;
use crate::command_bus::{bus::Command, error::CommandBusError};
use crate::events::FragmentPublishedEvent;
use anyhow::Context;
use commons::{commands::CommandType, id::Id};
use storage::{
    active::fragment::ActiveFragment,
    model::fragment::{Fragment, FragmentState},
};
use tap::TapFallible;

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
impl Command for PublishFragmentCommand {
    type Event = FragmentPublishedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::PublishFragment
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        //actor.is_user()
        todo!()
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

        match (fragment.state(), fragment.is_fork()) {
            (FragmentState::Draft, false) => Ok(()),
            (FragmentState::Approved, true) => Ok(()),
            (FragmentState::Draft, true) => Err(PublishFragmentCommandError::InvalidState(
                "forks need to be aprovved to be published",
            )),
            (FragmentState::Published, _) => Err(PublishFragmentCommandError::InvalidState(
                "fragment is already published",
            )),
            (_, true) => Err(PublishFragmentCommandError::InvalidState(
                "Fork can not be published",
            )),
            (_, false) => Err(PublishFragmentCommandError::InvalidState(
                "Fragment can not be published",
            )),
        }?;

        Ok(fragment
            .set_state(FragmentState::Published)
            .set_last_modified_at(ctx.clock().now())
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
            timestamp: *value.last_modified_at(),
            actor: commons::actor::Actor::User(*value.author_id()),
        }
    }
}