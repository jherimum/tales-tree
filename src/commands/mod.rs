pub mod create_tale;
use self::create_tale::CreateTaleCommandError;
use crate::{actor::Actor, storage::StorageError, Id, ReviewAction};
use derive_getters::Getters;
use sqlx::PgPool;

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error(transparent)]
    CreateTaleCommand(#[from] CreateTaleCommandError),

    #[error(transparent)]
    Storage(#[from] StorageError),
}

pub struct CommandBus {
    pool: PgPool,
}

impl CommandBus {
    pub async fn execute<C, O, E>(&self, actor: &Actor, command: C) -> Result<O, CommandBusError>
    where
        C: CommandHandler<Output = O, Error = E>,
        E: Into<CommandBusError>,
    {
        command
            .handle(&CommandHandlerContext {
                pool: &self.pool,
                actor: actor,
            })
            .await
            .map_err(Into::into)
    }
}

#[derive(Getters)]
pub struct CommandHandlerContext<'ctx> {
    pool: &'ctx PgPool,
    actor: &'ctx Actor,
}

#[async_trait::async_trait]
pub trait CommandHandler {
    type Error;
    type Output;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx CommandHandlerContext,
    ) -> Result<Self::Output, Self::Error>;
}

pub struct ForkTaleCommand {
    pub author: Id,
    pub parent: Id,
    pub content: String,
}

pub struct ReviewTaleCommand {
    pub tale: Id,
    pub reviewer: Id,
    pub comment: Option<String>,
    pub action: ReviewAction,
}

pub struct UpdateTaleCommand {
    pub author: Id,
    pub tale: Id,
    pub content: String,
}

pub struct CommentReviewCommand {
    pub review: Id,
    pub user: Id,
    pub comment: String,
}
