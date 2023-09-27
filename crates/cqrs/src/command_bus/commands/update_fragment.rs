use crate::command_bus::{bus::Command, bus::CommandHandlerContext, error::CommandBusError};
use crate::events::FragmentUpdatedEvent;
use commons::{
    actor::ActorTrait,
    commands::CommandType,
    id::{Id, IdGenerator},
    time::Clock,
};
use derive_getters::Getters;
use storage::{active::fragment::ActiveFragment, model::fragment::Fragment};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize, Getters)]
#[builder(setter(into))]
pub struct UpdateFragmentCommand {
    fragment_id: Id,
    content: String,
    end: bool,
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
impl Command for UpdateFragmentCommand {
    type Event = FragmentUpdatedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::UpdateFragment
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
        let user = ctx.actor().id().unwrap();

        let fragment = Fragment::find(ctx.pool(), &self.fragment_id).await?.ok_or(
            UpdateFragmentCommandError::FragmentNotFound(self.fragment_id),
        )?;

        if !fragment.is_author(user) {
            return Err(UpdateFragmentCommandError::UserWithoutPermission(user).into());
        }

        if !fragment.is_editable() {
            return Err(UpdateFragmentCommandError::NonEditableFragment(self.fragment_id).into());
        }

        Ok(fragment
            .set_content(self.content.clone())
            .set_last_modified_at(ctx.clock().now())
            .set_end(self.end)
            .update(ctx.tx().as_mut())
            .await
            .map(|f| Some(f.into()))
            .tap_err(|e| {
                tracing::error!("Failed to update fragment [{:?}]: {e}", self.fragment_id)
            })?)
    }

    fn supports<A: commons::actor::ActorTrait>(&self, actor: &A) -> bool {
        //actor.is_user()
        todo!()
    }
}

impl From<Fragment> for FragmentUpdatedEvent {
    fn from(value: Fragment) -> Self {
        FragmentUpdatedEvent {
            fragment_id: *value.id(),
            content: value.content().clone(),
            timestamp: *value.last_modified_at(),
            end: *value.end(),
        }
    }
}
