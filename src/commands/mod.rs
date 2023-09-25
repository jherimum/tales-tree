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
    clock::Clock,
    events::Event,
    id::{Id, IdGenerator},
    storage::{
        active::{event::ActiveEvent, task::ActiveTask},
        model::event::DbEvent,
        model::task::TaskBuilder,
        StorageError,
    },
    DateTime,
};
use chrono::Utc;
use serde::Serialize;
use sqlx::{postgres::any::AnyConnectionBackend, PgPool, Postgres, Transaction, Type};
use std::{fmt::Debug, sync::Arc};
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

#[async_trait::async_trait]
pub trait CommandBus {
    async fn dispatch<C>(
        &self,
        actor: &Actor,
        command: C,
        schedule_to: Option<DateTime>,
    ) -> Result<Id, CommandBusError>
    where
        C: CommandHandler + Debug + Serialize + Command + Send + Sync;

    async fn async_execute<C>(&self, actor: &Actor, command: C) -> Result<(), CommandBusError>
    where
        C: CommandHandler + 'static + Send + Sync + Debug + Command;

    async fn execute<C>(&self, actor: &Actor, command: C) -> Result<(), CommandBusError>
    where
        C: CommandHandler + Send + Sync + Debug + Command;
}

#[derive(Clone)]
pub struct SimpleCommandBus {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
}

impl SimpleCommandBus {
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, ids: Arc<dyn IdGenerator>) -> Self {
        Self { pool, clock, ids }
    }
}

#[async_trait::async_trait]
impl CommandBus for SimpleCommandBus {
    async fn dispatch<C>(
        &self,
        actor: &Actor,
        command: C,
        schedule_to: Option<DateTime>,
    ) -> Result<Id, CommandBusError>
    where
        C: CommandHandler + Debug + Serialize + Command + Send + Sync,
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

    async fn async_execute<C>(&self, actor: &Actor, command: C) -> Result<(), CommandBusError>
    where
        C: CommandHandler + 'static + Send + Sync + Debug + Command,
    {
        tokio::spawn(
            InnerExecutor {
                pool: self.pool.clone(),
                actor: actor.clone(),
                command,
                clock: self.clock.clone(),
                ids: self.ids.clone(),
            }
            .execute(),
        );

        Ok(())
    }

    async fn execute<C>(&self, actor: &Actor, command: C) -> Result<(), CommandBusError>
    where
        C: CommandHandler + Send + Sync + Debug + Command,
    {
        InnerExecutor {
            pool: self.pool.clone(),
            actor: actor.clone(),
            command,
            clock: self.clock.clone(),
            ids: self.ids.clone(),
        }
        .execute()
        .await
    }
}

struct InnerExecutor<C> {
    pool: PgPool,
    actor: Actor,
    command: C,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
}

impl<C: CommandHandler + Debug + Send + Sync + Command> InnerExecutor<C> {
    pub async fn execute(self) -> Result<(), CommandBusError> {
        if !self.command.supports(&self.actor) {
            tracing::error!(
                "Actor [{:?}] is not allowed to execute command [{:?}]",
                self.actor,
                self.command.command_type()
            );
            return Err(CommandBusError::ActorNotSupported(self.actor.clone()));
        };

        let mut ctx =
            CommandHandlerContext::new(&self.pool, &self.actor, self.clock, self.ids).await?;
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
                Ok(())
            }
            Err(e) => {
                ctx.tx
                    .as_mut()
                    .rollback()
                    .await
                    .tap_err(|e| tracing::error!("Failed to rollback tx: {e}"))?;
                Err(e.into())
            }
        }
    }
}

pub struct CommandHandlerContext<'ctx> {
    pool: PgPool,
    actor: Actor,
    tx: Transaction<'ctx, Postgres>,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
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

    pub fn clock(&self) -> &dyn Clock {
        self.clock.as_ref()
    }

    pub fn ids(&self) -> &dyn IdGenerator {
        self.ids.as_ref()
    }

    pub async fn new(
        pool: &PgPool,
        actor: &Actor,
        clock: Arc<dyn Clock>,
        ids: Arc<dyn IdGenerator>,
    ) -> Result<CommandHandlerContext<'ctx>, CommandBusError> {
        Ok(Self {
            pool: pool.clone(),
            actor: actor.clone(),
            tx: pool.begin().await.map_err(CommandBusError::from)?,
            clock,
            ids,
        })
    }
}

pub trait Command {
    fn command_type(&self) -> CommandType;
}

#[async_trait::async_trait]
pub trait CommandHandler {
    type Event: Event;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError>;

    fn supports(&self, actor: &Actor) -> bool;
}
