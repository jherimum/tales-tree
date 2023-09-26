use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    events::FragmentForkedEvent,
    id::Id,
    storage::{
        active::{fragment::ActiveFragment, user::ActiveUser},
        model::fragment::{Fragment, FragmentBuilder, FragmentState},
        model::user::User,
    },
};
use chrono::Utc;
use tap::TapFallible;

impl Command for ForkFragmentCommand {
    fn command_type(&self) -> CommandType {
        CommandType::ForkFragment
    }
}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct ForkFragmentCommand {
    fragment_id: Id,
    parent_fragment_id: Id,
    content: String,
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
impl CommandHandler for ForkFragmentCommand {
    type Event = FragmentForkedEvent;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError> {
        let user = User::try_from(ctx.actor())?;
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

        if parent_frag.is_author(&user) {
            return Err(
                ForkFragmentCommandError::Forbidden("Cannot fork your own fragment").into(),
            );
        }

        if !user.is_friend(ctx.pool(), *parent_frag.author_id()).await? {
            return Err(ForkFragmentCommandError::Forbidden(
                "You must be friend with the fragment author",
            )
            .into());
        }

        Ok(FragmentBuilder::default()
            .id(self.fragment_id)
            .author_id(*user.id())
            .content(self.content.clone())
            .parent_id(Some(self.parent_fragment_id))
            .state(FragmentState::Draft)
            .created_at(Utc::now().naive_utc())
            .last_modified_at(Utc::now().naive_utc())
            .path(parent_frag.path().append(self.parent_fragment_id))
            .build()
            .tap_err(|e| tracing::error!("Failed to build fragment: {e}"))
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .map(Into::into)
            .tap_err(|e| tracing::error!("Failed to save fragment: {e}"))?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}

impl From<Fragment> for Option<FragmentForkedEvent> {
    fn from(value: Fragment) -> Self {
        Some(FragmentForkedEvent {
            fragment_id: *value.id(),
            user_id: *value.author_id(),
            parent_fragment_id: value.parent_id().unwrap(),
            content: value.content().clone(),
            timestamp: *value.created_at(),
        })
    }
}
