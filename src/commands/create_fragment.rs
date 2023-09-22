use super::{Command, CommandBusError, CommandHandler, CommandHandlerContext, CommandType};
use crate::{
    actor::Actor,
    storage::{
        fragment::{Fragment, FragmentBuilder, FragmentState},
        user::User,
    },
    Id,
};
use chrono::Utc;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateFragmentCommandError {}

#[derive(Debug, derive_builder::Builder, serde::Deserialize, serde::Serialize)]
pub struct CreateFragmentCommand {
    pub id: Id,
    pub content: String,
}

impl Command for CreateFragmentCommand {}

#[async_trait::async_trait]
impl CommandHandler for CreateFragmentCommand {
    type Output = Fragment;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
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

    fn command_type(&self) -> CommandType {
        CommandType::CreateFragment
    }
}
