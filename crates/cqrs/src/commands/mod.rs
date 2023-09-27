pub mod create_fragment;
pub mod dislike_fragment;
pub mod follow_user;
pub mod fork_fragment;

pub mod like_fragment;
pub mod publish_fragment;
pub mod review_fork;
pub mod unfollow_user;
pub mod update_fragment;

use commons::{
    actor::ActorTrait,
    commands::CommandType,
    id::{Id, IdGenerator},
    time::{Clock, DateTime},
};
use serde::Serialize;
use sqlx::{postgres::any::AnyConnectionBackend, PgPool, Postgres, Transaction};
use std::{fmt::Debug, sync::Arc};
use storage::{
    active::{event::ActiveEvent, task::ActiveTask},
    model::{
        event::{DbEvent, DbEventBuilder, EventData},
        task::TaskBuilder,
    },
    StorageError,
};
use tap::TapFallible;

use crate::events::Event;

use self::{
    create_fragment::CreateFragmentCommandError, dislike_fragment::DislikeFragmentCommandError,
    fork_fragment::ForkFragmentCommandError, like_fragment::LikeFragmentCommandError,
    publish_fragment::PublishFragmentCommandError, review_fork::ReviewForkCommandError,
    update_fragment::UpdateFragmentCommandError,
};

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
    ActorNotSupported(Box<dyn ActorTrait>),

    #[error(transparent)]
    Tx(#[from] sqlx::Error),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait::async_trait]
pub trait CommandBus {
    async fn dispatch<C, A>(
        &self,
        actor: A,
        command: C,
        schedule_to: Option<DateTime>,
    ) -> Result<Id, CommandBusError>
    where
        C: Command + Serialize,
        A: ActorTrait + 'static;

    async fn async_execute<C, A>(&self, actor: A, command: C) -> Result<(), CommandBusError>
    where
        C: Command + 'static,
        A: ActorTrait + 'static + Clone;

    async fn execute<C, A>(&self, actor: A, command: C) -> Result<(), CommandBusError>
    where
        C: Command,
        A: ActorTrait + Clone + 'static;
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
    async fn dispatch<C, A>(
        &self,
        actor: A,
        command: C,
        schedule_to: Option<DateTime>,
    ) -> Result<Id, CommandBusError>
    where
        C: Command + Serialize,
        A: ActorTrait + 'static,
    {
        if !command.supports(&actor) {
            tracing::error!("Actor [{actor:?}] is not allowed to execute command [{command:?}]");
            return Err(CommandBusError::ActorNotSupported(Box::new(actor)));
        };

        Ok(TaskBuilder::default()
            .id(Id::new())
            .command_type(command.command_type())
            .commnad_data(command.into())
            .actor_type(actor.actor_type())
            .actor_id(actor.id())
            .scheduled_at(schedule_to.unwrap_or(DateTime::now()))
            .build()
            .map_err(anyhow::Error::from)?
            .save(&self.pool)
            .await
            .map(|t| *t.id())?)
    }

    async fn async_execute<C, A>(&self, actor: A, command: C) -> Result<(), CommandBusError>
    where
        C: Command + 'static,
        A: ActorTrait + 'static + Clone,
    {
        let executor = InnerExecutor::new(
            self.pool.clone(),
            actor,
            command,
            self.clock.clone(),
            self.ids.clone(),
        );
        tokio::spawn(executor.execute());

        Ok(())
    }

    async fn execute<C, A>(&self, actor: A, command: C) -> Result<(), CommandBusError>
    where
        C: Command,
        A: ActorTrait + Clone + 'static,
    {
        InnerExecutor {
            pool: self.pool.clone(),
            actor: actor,
            command,
            clock: self.clock.clone(),
            ids: self.ids.clone(),
        }
        .execute()
        .await
    }
}

struct InnerExecutor<C, A> {
    pool: PgPool,
    actor: A,
    command: C,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
}

impl<C, A> InnerExecutor<C, A>
where
    C: Command,
    A: ActorTrait + Clone + 'static,
{
    pub fn new(
        pool: PgPool,
        actor: A,
        command: C,
        clock: Arc<dyn Clock>,
        ids: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            pool,
            actor,
            command,
            clock,
            ids,
        }
    }

    pub async fn execute(self) -> Result<(), CommandBusError> {
        if !self.command.supports(&self.actor) {
            tracing::error!(
                "Actor [{:?}] is not allowed to execute command [{:?}]",
                self.actor,
                self.command.command_type()
            );

            let a = self.actor.clone();
            return Err(CommandBusError::ActorNotSupported(Box::new(a)));
        };

        let mut ctx = CommandHandlerContext::new(
            &self.pool,
            &self.actor,
            self.clock.as_ref(),
            self.ids.as_ref(),
        )
        .await?;
        let result = self
            .command
            .handle(&mut ctx)
            .await
            .tap_ok(|_| tracing::info!("Command [{:?}] handled successfully", self.command))
            .tap_err(|e| tracing::error!("Failed to handle command [{:?}]: {e}", self.command));

        match result {
            Ok(result) => {
                if result.is_some() {
                    self.save_event(&mut ctx, result.unwrap()).await?;
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
                Err(e)
            }
        }
    }

    async fn save_event<'ctx>(
        &self,
        ctx: &mut CommandHandlerContext<'ctx>,
        event: impl Event,
    ) -> Result<DbEvent, StorageError> {
        DbEventBuilder::default()
            .id(ctx.ids().new_id())
            .timestamp(event.timestamp())
            .event_type(event.event_type())
            .event_data(EventData::from(event))
            .build()
            .unwrap()
            .save(ctx.tx().as_mut())
            .await
    }
}

pub struct CommandHandlerContext<'ctx> {
    pool: &'ctx PgPool,
    actor: &'ctx dyn ActorTrait,
    tx: Transaction<'ctx, Postgres>,
    clock: &'ctx dyn Clock,
    ids: &'ctx dyn IdGenerator,
}

impl<'ctx> CommandHandlerContext<'ctx> {
    pub fn pool(&self) -> &PgPool {
        self.pool
    }

    pub fn actor(&self) -> &dyn ActorTrait {
        self.actor
    }

    pub fn tx(&mut self) -> &mut Transaction<'ctx, Postgres> {
        &mut self.tx
    }

    pub fn clock(&self) -> &dyn Clock {
        self.clock
    }

    pub fn ids(&self) -> &dyn IdGenerator {
        self.ids
    }

    pub async fn new(
        pool: &'ctx PgPool,
        actor: &'ctx dyn ActorTrait,
        clock: &'ctx dyn Clock,
        ids: &'ctx dyn IdGenerator,
    ) -> Result<CommandHandlerContext<'ctx>, CommandBusError> {
        Ok(Self {
            pool,
            actor,
            tx: pool.begin().await.map_err(CommandBusError::from)?,
            clock,
            ids,
        })
    }
}

#[async_trait::async_trait]
pub trait Command: Send + Sync + Debug {
    type Event: Event;

    fn command_type(&self) -> CommandType;

    async fn handle(
        &self,
        ctx: &mut CommandHandlerContext,
    ) -> Result<Option<Self::Event>, CommandBusError>;

    fn supports<A>(&self, actor: &A) -> bool
    where
        A: ActorTrait;
}
