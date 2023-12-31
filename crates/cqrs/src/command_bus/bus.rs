use super::{command::Command, error::CommandBusError};
use crate::events::Event;
use commons::{
    actor::ActorTrait,
    id::{Id, IdGenerator},
    time::{Clock, DateTime},
};
use serde::Serialize;
use sqlx::{postgres::any::AnyConnectionBackend, PgPool, Postgres, Transaction};
use std::{marker::PhantomData, sync::Arc};
use storage::{
    model::{
        event::{DbEvent, DbEventBuilder, EventData},
        task::TaskBuilder,
    },
    query::{event::QueryEvent, task::QueryTask},
    StorageError,
};
use tap::TapFallible;

#[derive(Clone)]
pub struct CommandBus {
    pool: PgPool,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
}

impl CommandBus {
    pub fn new(pool: PgPool, clock: Arc<dyn Clock>, ids: Arc<dyn IdGenerator>) -> Self {
        Self { pool, clock, ids }
    }
}

impl CommandBus {
    pub async fn dispatch<C, A>(
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
            .scheduled_at(schedule_to.unwrap_or_else(DateTime::now))
            .build()
            .map_err(anyhow::Error::from)?
            .save(&self.pool)
            .await
            .map(|t| *t.id())?)
    }

    pub async fn async_execute<C, A, EV>(&self, actor: A, command: C) -> Result<(), CommandBusError>
    where
        C: Command<Event = EV> + 'static,
        EV: Event + Serialize + 'static,
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

    pub async fn execute<C, A, EV>(&self, actor: A, command: C) -> Result<(), CommandBusError>
    where
        C: Command<Event = EV>,
        A: ActorTrait + Clone + 'static,
        EV: Event + Serialize,
    {
        InnerExecutor::new(
            self.pool.clone(),
            actor,
            command,
            self.clock.clone(),
            self.ids.clone(),
        )
        .execute()
        .await
    }
}

struct InnerExecutor<C, A, EV> {
    pool: PgPool,
    actor: A,
    command: C,
    clock: Arc<dyn Clock>,
    ids: Arc<dyn IdGenerator>,
    ev: PhantomData<EV>,
}

impl<C, A, EV> InnerExecutor<C, A, EV>
where
    C: Command<Event = EV>,
    A: ActorTrait + Clone + 'static,
    EV: Event + Serialize,
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
            ev: PhantomData::<EV>,
        }
    }

    pub async fn execute(self) -> Result<(), CommandBusError> {
        if !self.command.supports(&self.actor) {
            tracing::error!(
                "Actor [{:?}] is not allowed to execute command [{:?}]",
                self.actor,
                self.command.command_type()
            );

            return Err(CommandBusError::ActorNotSupported(Box::new(
                self.actor.clone(),
            )));
        };

        let mut ctx = Ctx::new(
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
                if let Some(event) = result {
                    self.save_event(&mut ctx, event)
                        .await
                        .tap_err(|e| tracing::error!("Failed to save event: {e}"))?;
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

    async fn save_event<'ctx, E: Event + Serialize>(
        &self,
        ctx: &mut Ctx<'ctx>,
        event: E,
    ) -> Result<DbEvent, StorageError> {
        DbEventBuilder::default()
            .id(ctx.ids().new_id())
            .timestamp(event.timestamp())
            .event_type(event.event_type())
            .event_data(EventData::from(&event))
            .actor_id(event.actor().id())
            .actor_type((&event.actor()).into())
            .build()
            .unwrap()
            .save(ctx.tx().as_mut())
            .await
    }
}

pub struct Ctx<'ctx> {
    pool: &'ctx PgPool,
    actor: &'ctx dyn ActorTrait,
    tx: Transaction<'ctx, Postgres>,
    clock: &'ctx dyn Clock,
    ids: &'ctx dyn IdGenerator,
}

impl<'ctx> Ctx<'ctx> {
    pub async fn new(
        pool: &'ctx PgPool,
        actor: &'ctx dyn ActorTrait,
        clock: &'ctx dyn Clock,
        ids: &'ctx dyn IdGenerator,
    ) -> Result<Ctx<'ctx>, CommandBusError> {
        Ok(Self {
            pool,
            actor,
            tx: pool.begin().await.map_err(CommandBusError::from)?,
            clock,
            ids,
        })
    }
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
}
