pub mod create_fragment;
pub mod follow_user;
pub mod fork_fragment;
pub mod like_fragment;
pub mod publish_fragment;
pub mod review_fork;
pub mod update_fragment;

use crate::{actor::Actor, storage::StorageError};
use sqlx::{postgres::any::AnyConnectionBackend, PgPool, Postgres, Transaction};
use std::fmt::{Debug, Display};
use tap::TapFallible;

use self::{
    create_fragment::CreateFragmentCommandError, fork_fragment::ForkFragmentCommandError,
    like_fragment::LikeOrDislikeFragmentCommandError,
    publish_fragment::PublishFragmentCommandError, review_fork::ReviewForkCommandError,
    update_fragment::UpdateFragmentCommandError,
};

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error(transparent)]
    LikeFragmentCommand(#[from] LikeOrDislikeFragmentCommandError),

    #[error(transparent)]
    CreateFragmentCommand(#[from] CreateFragmentCommandError),

    #[error(transparent)]
    ForkFragmentCommand(#[from] ForkFragmentCommandError),

    #[error(transparent)]
    PublishFragmentCommand(#[from] PublishFragmentCommandError),

    #[error(transparent)]
    UpdateFragmentCommand(#[from] UpdateFragmentCommandError),

    #[error(transparent)]
    ReviewForkCommand(#[from] ReviewForkCommandError),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error("Actor type forbidden")]
    ActorNotSupported(Actor),

    #[error(transparent)]
    Tx(#[from] sqlx::Error),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub struct CommandBus {
    pool: PgPool,
}

impl CommandBus {
    pub async fn execute<C, E>(
        &self,
        actor: &Actor,
        command: C,
    ) -> Result<C::Output, CommandBusError>
    where
        C: CommandHandler + Display + Debug,
        E: Into<CommandBusError> + Display,
    {
        if !command.supports(actor) {
            tracing::error!("Actor [{actor:?}] is not allowed to execute command [{command:?}]");
            return Err(CommandBusError::ActorNotSupported(actor.clone()));
        };

        let ctx = &mut CommandHandlerContext::new(&self.pool, &actor).await?;
        let result = command
            .handle(ctx)
            .await
            .tap_ok(|_| tracing::info!("Command [{command:?}] handled successfully"))
            .tap_err(|e| tracing::error!("Failed to handle command [{command:?}]: {e}"));

        match result {
            Ok(result) => {
                ctx.tx().as_mut().commit().await?;
                Ok(result)
            }
            Err(e) => {
                ctx.tx.as_mut().rollback().await?;
                Err(e)
            }
        }
    }
}

pub struct CommandHandlerContext<'ctx> {
    pool: &'ctx PgPool,
    actor: &'ctx Actor,
    tx: Transaction<'ctx, Postgres>,
}

impl<'ctx> CommandHandlerContext<'ctx> {
    pub fn pool(&self) -> &PgPool {
        self.pool
    }

    pub fn actor(&self) -> &Actor {
        self.actor
    }

    pub fn tx(&mut self) -> &mut Transaction<'ctx, Postgres> {
        &mut self.tx
    }

    pub async fn new(
        pool: &'ctx PgPool,
        actor: &'ctx Actor,
    ) -> Result<CommandHandlerContext<'ctx>, CommandBusError> {
        Ok(Self {
            pool: &pool,
            actor: actor,
            tx: pool.begin().await.map_err(CommandBusError::from)?,
        })
    }
}

#[async_trait::async_trait]
pub trait CommandHandler {
    type Output: Debug;

    async fn handle<'ctx>(
        &self,
        ctx: &'ctx mut CommandHandlerContext,
    ) -> Result<Self::Output, CommandBusError>;

    fn supports(&self, actor: &Actor) -> bool;
}
