use sqlx::PgPool;
use std::sync::Arc;
use tales_tree::{
    actor::Actor, clock::Clock, commands::CommandHandlerContext, id::IdGenerator,
    storage::user::User,
};

pub async fn create_context<C: Clock + 'static, I: IdGenerator + 'static>(
    pool: &PgPool,
    user: User,
    clock: C,
    ids: I,
) -> CommandHandlerContext {
    CommandHandlerContext::new(pool, &Actor::User(user), Arc::new(clock), Arc::new(ids))
        .await
        .unwrap()
}
