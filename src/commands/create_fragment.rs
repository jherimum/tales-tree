use super::{CommandBusError, CommandHandler, CommandHandlerContext};
use crate::{
    actor::Actor,
    storage::{
        fragment::{Fragment, FragmentBuilder, FragmentState},
        user::User,
    },
    Id,
};
use chrono::Utc;
use derive_builder::Builder;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateFragmentCommandError {}

#[derive(Debug, Builder)]
pub struct CreateFragmentCommand {
    pub id: Id,
    pub content: String,
}

#[async_trait::async_trait]
impl CommandHandler for CreateFragmentCommand {
    type Output = Fragment;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError> {
        Ok(FragmentBuilder::default()
            .id(self.id)
            .author_id(User::try_from(ctx.actor()).unwrap())
            .content(self.content.clone())
            .state(FragmentState::Draft)
            .created_at(Utc::now().naive_utc())
            .last_modified_at(Utc::now().naive_utc())
            .build()
            .tap_err(|e| tracing::error!("Failed to build fragment: {e}"))
            .map_err(anyhow::Error::from)?
            .save(ctx.tx().as_mut())
            .await
            .tap_err(|e| tracing::error!("Failed to save fragment:{e}"))?)
    }

    fn supports(&self, actor: &Actor) -> bool {
        actor.is_user()
    }
}
