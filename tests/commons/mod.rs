use sqlx::PgPool;
use std::sync::Arc;
use tales_tree::{
    actor::Actor, clock::Clock, commands::CommandHandlerContext, id::IdGenerator,
    storage::model::user::User,
};

pub async fn create_context<'ctx, C: Clock + 'static, I: IdGenerator + 'static>(
    pool: &PgPool,
    user: &User,
    clock: C,
    ids: I,
) -> CommandHandlerContext<'ctx> {
    CommandHandlerContext::new(
        pool,
        &Actor::User(user.clone()),
        Arc::new(clock),
        Arc::new(ids),
    )
    .await
    .unwrap()
}
