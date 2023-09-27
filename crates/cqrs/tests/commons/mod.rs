use commons::{id::IdGenerator, time::Clock};
use cqrs::commands::CommandHandlerContext;
use sqlx::PgPool;
use storage::model::user::User;

pub async fn create_context<'ctx, C: Clock, I: IdGenerator>(
    pool: &'ctx PgPool,
    user: &'ctx User,
    clock: &'ctx C,
    ids: &'ctx I,
) -> CommandHandlerContext<'ctx> {
    CommandHandlerContext::new(pool, user, clock, ids)
        .await
        .unwrap()
}
