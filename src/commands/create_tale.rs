use super::{CommandHandler, CommandHandlerContext};
use crate::{
    storage::{
        tale::{Tale, TaleBuilder, TaleBuilderError, TaleState},
        StorageError,
    },
    Id, User,
};
use chrono::Utc;
use derive_builder::Builder;

#[derive(Debug, thiserror::Error)]
pub enum CreateTaleCommandError {
    #[error(transparent)]
    TaleBuild(#[from] TaleBuilderError),

    #[error("Only users can create a tale")]
    InvalidActor,

    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Debug, Builder)]
pub struct CreateTaleCommand {
    pub id: Id,
    pub content: String,
}

#[async_trait::async_trait]
impl CommandHandler for CreateTaleCommand {
    type Error = CreateTaleCommandError;
    type Output = Tale;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx CommandHandlerContext,
    ) -> Result<Self::Output, Self::Error> {
        let author: Option<User> = ctx.actor().into();
        let author = author.ok_or(CreateTaleCommandError::InvalidActor)?;

        let tale = TaleBuilder::default()
            .id(self.id)
            .author_id(author)
            .content(self.content.clone())
            .created_at(Utc::now())
            .state(TaleState::Draft)
            .build()?;

        Ok(tale.save(ctx.pool).await?)
    }
}
