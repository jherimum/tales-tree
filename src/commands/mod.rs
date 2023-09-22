pub mod create_fragment;
pub mod dislike_fragment;
pub mod follow_user;
pub mod fork_fragment;

pub mod like_fragment;
pub mod publish_fragment;
pub mod review_fork;
pub mod unfollow_user;
pub mod update_fragment;

use crate::{
    actor::Actor,
    events::Event,
    id::Id,
    storage::{event::DbEvent, task::TaskBuilder, StorageError},
    DateTime,
};
use chrono::Utc;
use serde::Serialize;
use sqlx::{postgres::any::AnyConnectionBackend, PgPool, Postgres, Transaction, Type};
use std::fmt::Debug;
use tap::TapFallible;

use self::{
    create_fragment::CreateFragmentCommandError, dislike_fragment::DislikeFragmentCommandError,
    fork_fragment::ForkFragmentCommandError, like_fragment::LikeFragmentCommandError,
    publish_fragment::PublishFragmentCommandError, review_fork::ReviewForkCommandError,
    update_fragment::UpdateFragmentCommandError,
};

#[derive(Debug, Type, Clone)]
pub enum CommandType {
    CreateFragment,
    FollowUser,
    UnfollowUser,
    LikeFragment,
    DislikeFragment,
    ForkFragment,
    PublishFragment,
    UpdateFragment,
    ReviewFork,
}

#[derive(Debug, thiserror::Error)]
pub enum CommandBusError {
    #[error(transparent)]
    DislikeFragmentCommand(#[from] DislikeFragmentCommandError),

    #[error(transparent)]
    LikeFragmentCommand(#[from] LikeFragmentCommandError),

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

#[derive(Clone)]
pub struct CommandBus {
    pool: PgPool,
}

impl CommandBus {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn dispatch<C, E>(
        &self,
        actor: &Actor,
        command: C,
        schedule_to: Option<DateTime>,
    ) -> Result<Id, CommandBusError>
    where
        C: CommandHandler,
        E: Into<CommandBusError>,
    {
        if !command.supports(actor) {
            tracing::error!("Actor [{actor:?}] is not allowed to execute command [{command:?}]");
            return Err(CommandBusError::ActorNotSupported(actor.clone()));
        };

        Ok(TaskBuilder::default()
            .id(Id::new())
            .command_type(command.command_type())
            .commnad_data(command.into())
            .actor_type(actor.into())
            .actor_id(actor.into())
            .scheduled_at(schedule_to.unwrap_or(Utc::now().naive_utc()))
            .build()
            .map_err(anyhow::Error::from)?
            .save(&self.pool)
            .await
            .map(|t| *t.id())?)
    }

    pub async fn async_execute<C, E>(
        &self,
        actor: &Actor,
        command: C,
    ) -> Result<(), CommandBusError>
    where
        C: CommandHandler + Send + Sync + Clone + 'static,
        E: Into<CommandBusError>,
    {
        tokio::spawn(
            Executor {
                pool: self.pool.clone(),
                actor: actor.clone(),
                command: command.clone(),
            }
            .execute(),
        );

        Ok(())
    }

    pub async fn execute<C, E>(
        &self,
        actor: &Actor,
        command: C,
    ) -> Result<Option<C::Event>, CommandBusError>
    where
        C: CommandHandler + Send + Sync,
        E: Into<CommandBusError>,
    {
        Executor {
            pool: self.pool.clone(),
            actor: actor.clone(),
            command,
        }
        .execute()
        .await
    }
}

pub struct Executor<C> {
    pool: PgPool,
    actor: Actor,
    command: C,
}

impl<C: CommandHandler> Executor<C> {
    pub fn new(pool: PgPool, actor: Actor, command: C) -> Self {
        Self {
            pool,
            actor,
            command,
        }
    }

    pub async fn execute(self) -> Result<Option<C::Event>, CommandBusError> {
        if !self.command.supports(&self.actor) {
            tracing::error!(
                "Actor [{:?}] is not allowed to execute command [{:?}]",
                self.actor,
                self.command.command_type()
            );
            return Err(CommandBusError::ActorNotSupported(self.actor.clone()));
        };

        let mut ctx = CommandHandlerContext::new(&self.pool, &self.actor).await?;
        let result = self
            .command
            .handle(&mut ctx)
            .await
            .tap_ok(|_| tracing::info!("Command [{:?}] handled successfully", self.command))
            .tap_err(|e| tracing::error!("Failed to handle command [{:?}]: {e}", self.command));

        match result {
            Ok(result) => {
                if let Some(event) = &result {
                    DbEvent::from(event.clone())
                        .save(ctx.tx().as_mut())
                        .await
                        .tap_err(|e| tracing::error!("Failed to save event:{e}"))
                        .tap_ok(|e| tracing::info!("Event [{e:?}] saved successfully."))?;
                }
                ctx.tx()
                    .as_mut()
                    .commit()
                    .await
                    .tap_err(|e| tracing::error!("Failed to commit tx: {e}"))?;
                Ok(result)
            }
            Err(e) => {
                ctx.tx
                    .as_mut()
                    .rollback()
                    .await
                    .tap_err(|e| tracing::error!("Failed to rollback tx: {e}"))?;
                Err(e)
            }
        }
    }
}

pub struct CommandHandlerContext<'ctx> {
    pool: PgPool,
    actor: Actor,
    tx: Transaction<'ctx, Postgres>,
}

impl<'ctx> CommandHandlerContext<'ctx> {
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn actor(&self) -> &Actor {
        &self.actor
    }

    pub fn tx(&mut self) -> &mut Transaction<'ctx, Postgres> {
        &mut self.tx
    }

    pub async fn new(
        pool: &PgPool,
        actor: &Actor,
    ) -> Result<CommandHandlerContext<'ctx>, CommandBusError> {
        Ok(Self {
            pool: pool.clone(),
            actor: actor.clone(),
            tx: pool.begin().await.map_err(CommandBusError::from)?,
        })
    }
}

pub trait Command {}

#[async_trait::async_trait]
pub trait CommandHandler: Debug + Send + Serialize
where
    Self: Command,
{
    type Event: Event + Debug + Send + Sync;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError>;

    fn supports(&self, actor: &Actor) -> bool;

    fn command_type(&self) -> CommandType;
}
