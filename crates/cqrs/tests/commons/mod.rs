use commons::{actor::Actor, id::IdGenerator, time::Clock};
use cqrs::commands::CommandHandlerContext;
use sqlx::PgPool;
use std::sync::Arc;
use storage::model::user::User;

pub async fn create_context<'ctx, C: Clock + 'static, I: IdGenerator + 'static>(
    pool: &PgPool,
    user: &User,
    clock: C,
    ids: I,
) -> CommandHandlerContext<'ctx> {
    CommandHandlerContext::new(
        pool,
        &Actor::User(*user.id()),
        Arc::new(clock),
        Arc::new(ids),
    )
    .await
    .unwrap()
}
