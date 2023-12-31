use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::FragmentUpdatedEvent;
use commons::actor::{Actor, ActorType};
use commons::fragment::Content;
use commons::{commands::CommandType, id::Id};
use derive_getters::Getters;
use storage::{model::fragment::Fragment, query::fragment::QueryFragment};
use tap::TapFallible;

use super::Command;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize, Getters)]
#[builder(setter(into))]
pub struct UpdateFragmentCommand {
    fragment_id: Id,
    content: Option<Content>,
    end: Option<bool>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum UpdateFragmentCommandError {
    #[error("Fragment not found: {0}")]
    FragmentNotFound(Id),

    #[error("{0}")]
    UserWithoutPermission(Id),

    #[error("Fragment is not editable")]
    NonEditableFragment(Id),

    #[error("Only forks can be ended")]
    NonEndabledFragment(Id),
}

#[async_trait::async_trait]
impl Command for UpdateFragmentCommand {
    type Event = FragmentUpdatedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::UpdateFragment
    }

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = ctx.actor().id().unwrap();

        let fragment = Fragment::find(ctx.pool(), &self.fragment_id)
            .await
            .tap_err(|e| tracing::error!("Failed to find fragment: {e}"))?
            .ok_or(UpdateFragmentCommandError::FragmentNotFound(
                self.fragment_id,
            ))?;

        if !fragment.is_editable() {
            return Err(UpdateFragmentCommandError::NonEditableFragment(self.fragment_id).into());
        }

        if !fragment.is_author(user) {
            return Err(UpdateFragmentCommandError::UserWithoutPermission(user).into());
        }

        // if self.end && !fragment.is_fork() {
        //     return Err(UpdateFragmentCommandError::NonEndabledFragment(self.fragment_id).into());
        // }

        Ok(fragment
            .set_content(self.content.clone().unwrap())
            .set_last_modified_at(ctx.clock().now())
            .set_end(self.end.unwrap())
            .update(ctx.tx().as_mut())
            .await
            .map(Into::into)
            .map(Some)
            .tap_err(|e| {
                tracing::error!("Failed to update fragment [{:?}]: {e}", self.fragment_id)
            })?)
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        ActorType::User == actor.actor_type()
    }
}

impl From<Fragment> for FragmentUpdatedEvent {
    fn from(value: Fragment) -> Self {
        Self {
            fragment_id: *value.id(),
            content: value.content().clone(),
            timestamp: *value.last_modified_at(),
            end: *value.end(),
            actor: Actor::User(*value.author_id()),
        }
    }
}
