use commons::{id::IdGenerator, time::Clock};
use cqrs::command_bus::bus::{Context, Ctx};
use sqlx::PgPool;
use storage::model::user::User;

pub async fn create_context<'ctx, C: Clock, I: IdGenerator>(
    pool: &'ctx PgPool,
    user: &'ctx User,
    clock: &'ctx C,
    ids: &'ctx I,
) -> impl Context<'ctx> {
    Ctx::new(pool, user, clock, ids).await.unwrap()
}
