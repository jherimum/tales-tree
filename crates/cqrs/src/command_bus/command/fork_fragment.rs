use super::Command;
use crate::command_bus::bus::Ctx;
use crate::command_bus::error::CommandBusError;
use crate::events::FragmentForkedEvent;
use commons::actor::ActorType;
use commons::fragment::Content;
use commons::{actor::ActorTrait, commands::CommandType, id::Id};
use storage::active::user::ActiveUser;
use storage::{
    active::fragment::ActiveFragment,
    model::fragment::{Fragment, FragmentBuilder},
};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
#[builder(setter(into))]
pub struct ForkFragmentCommand {
    fragment_id: Id,
    parent_fragment_id: Id,
    content: Content,
    end: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ForkFragmentCommandError {
    #[error("Parent fragment not found: {0}")]
    ParentFragmentNotFound(Id),

    #[error("{0}")]
    Forbidden(&'static str),

    #[error("{0}")]
    InvalidState(&'static str),
}

#[async_trait::async_trait]
impl Command for ForkFragmentCommand {
    type Event = FragmentForkedEvent;

    fn command_type(&self) -> CommandType {
        CommandType::ForkFragment
    }

    async fn handle<'ctx>(
        &self,
        ctx: &mut Ctx<'ctx>,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = ctx.actor().id().unwrap();
        let parent_frag = Fragment::find(ctx.pool(), &self.parent_fragment_id)
            .await
            .tap_err(|e| {
                tracing::error!("Failed to find fragment [{}]: {e}", self.parent_fragment_id)
            })?
            .ok_or(ForkFragmentCommandError::ParentFragmentNotFound(
                self.fragment_id,
            ))?;

        if !parent_frag.is_published() {
            return Err(
                ForkFragmentCommandError::InvalidState("Parent fragment is not published").into(),
            );
        }

        if parent_frag.is_author(user) {
            return Err(
                ForkFragmentCommandError::Forbidden("Cannot fork your own fragment").into(),
            );
        }

        if *parent_frag.end() {
            return Err(ForkFragmentCommandError::Forbidden("Cannot fork an end fragment").into());
        }

        let friend = parent_frag
            .author(ctx.pool())
            .await?
            .is_friend(ctx.pool(), user)
            .await?;

        if !friend {
            return Err(
                ForkFragmentCommandError::Forbidden("You must be fragment author friend").into(),
            );
        }

        let now = ctx.clock().now();
        Ok(FragmentBuilder::default()
            .id(self.fragment_id)
            .author_id(user)
            .content(self.content.clone())
            .parent_id(Some(self.parent_fragment_id))
            .end(self.end)
            .created_at(now)
            .last_modified_at(now)
            .path(parent_frag.path().append(self.parent_fragment_id))
            .build()
            .tap_err(|e| tracing::error!("Failed to build fragment: {e}"))
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .map(|f| Some(f.into()))
            .tap_err(|e| tracing::error!("Failed to save fragment: {e}"))?)
    }

    fn supports<A: ActorTrait>(&self, actor: &A) -> bool {
        ActorType::User == actor.actor_type()
    }
}

impl From<Fragment> for FragmentForkedEvent {
    fn from(value: Fragment) -> Self {
        Self {
            fragment_id: *value.id(),
            user_id: *value.author_id(),
            parent_fragment_id: value.parent_id().unwrap(),
            content: value.content().clone(),
            timestamp: *value.created_at(),
            end: *value.end(),
        }
    }
}
